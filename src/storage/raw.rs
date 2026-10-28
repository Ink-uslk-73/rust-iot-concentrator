//! src/storage/raw.rs - Raw CBOR fallback (satellite nodes).

use std::path::Path;
use serde::{Deserialize, Serialize};

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
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;
        ciborium::into_writer(point, &file).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<RawPoint>, String> {
        if !Path::new(&self.path).exists() {
            return Ok(Vec::new());
        }
        let file = std::fs::File::open(&self.path).map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        let mut reader = std::io::BufReader::new(file);
        loop {
            match ciborium::from_reader::<RawPoint, _>(&mut reader) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cbor_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let store = RawStore::new(dir.path().join("test.cbor"));
        store.append(&RawPoint { sensor_id: "s1".into(), value: 42.5, ts_ms: 1 }).unwrap();
        store.append(&RawPoint { sensor_id: "s2".into(), value: 43.5, ts_ms: 2 }).unwrap();
        let points = store.read_all().unwrap();
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].sensor_id, "s1");
        assert_eq!(points[0].value, 42.5);
    }
}

