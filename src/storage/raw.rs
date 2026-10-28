//! src/storage/raw.rs - Raw CBOR fallback + sync to TimescaleDB.

use std::path::Path;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawPoint {
    pub sensor_id: String,
    pub value: f64,
    pub ts_ms: u64,
}

pub struct RawStore {
    path: std::path::PathBuf,
}

impl RawStore {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn append(&self, point: &RawPoint) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let file = std::fs::OpenOptions::new()
            .create(true).append(true).open(&self.path)
            .map_err(|e| e.to_string())?;
        ciborium::into_writer(point, &file).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<RawPoint>, String> {
        if !Path::new(&self.path).exists() { return Ok(Vec::new()); }
        let file = std::fs::File::open(&self.path).map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        let mut rd = std::io::BufReader::new(file);
        loop {
            match ciborium::from_reader::<RawPoint, _>(&mut rd) {
                Ok(p) => out.push(p),
                Err(_) => break,
            }
        }
        Ok(out)
    }

    pub fn clear(&self) -> Result<(), String> {
        let _ = std::fs::remove_file(&self.path);
        Ok(())
    }

    /// Если pool = None (БД недоступна) -> Ok(0), Raw остаётся для будущей синхронизации.
    pub async fn sync_to_timescale(&self, pool: Option<&PgPool>) -> Result<usize, String> {
        let Some(pool) = pool else { return Ok(0); };
        let points = self.read_all()?;
        if points.is_empty() { return Ok(0); }

        let mut count = 0usize;
        for p in &points {
            let ts = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(p.ts_ms as i64)
                .unwrap_or_else(chrono::Utc::now);
            let res = sqlx::query(
                r#"INSERT INTO telemetry (ts, sensor_id, value, source) VALUES ($1,$2,$3,'raw')"#
            )
            .bind(ts).bind(&p.sensor_id).bind(p.value)
            .execute(pool).await;
            if let Err(e) = res {
                return Err(format!("insert telemetry: {}", e));
            }
            count += 1;
        }

        if count == points.len() {
            let _ = self.clear();
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cbor_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let s = RawStore::new(dir.path().join("t.cbor"));
        s.append(&RawPoint { sensor_id: "s1".into(), value: 42.5, ts_ms: 1 }).unwrap();
        s.append(&RawPoint { sensor_id: "s2".into(), value: 43.5, ts_ms: 2 }).unwrap();
        let pts = s.read_all().unwrap();
        assert_eq!(pts.len(), 2);
        assert_eq!(pts[0].sensor_id, "s1");
    }

    #[tokio::test]
    async fn sync_without_pool_is_noop() {
        let dir = tempfile::tempdir().unwrap();
        let s = RawStore::new(dir.path().join("t.cbor"));
        s.append(&RawPoint { sensor_id: "s".into(), value: 1.0, ts_ms: 1 }).unwrap();
        assert_eq!(s.sync_to_timescale(None).await.unwrap(), 0);
        assert_eq!(s.read_all().unwrap().len(), 1);
    }
}
