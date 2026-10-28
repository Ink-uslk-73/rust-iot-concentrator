//! src/storage/cache.rs - Redis hot-path cache.

use redis::AsyncCommands;

pub struct HotCache {
    client: redis::Client,
    ttl_sec: u64,
}

impl HotCache {
    pub fn new(url: &str, ttl_sec: u64) -> Result<Self, String> {
        let client = redis::Client::open(url).map_err(|e| e.to_string())?;
        Ok(Self { client, ttl_sec })
    }

    pub async fn set_last(&self, sensor_id: &str, value: f64) -> Result<(), String> {
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        let key = format!("sensor:last:{}", sensor_id);
        let _: () = conn.set_ex(key, value, self.ttl_sec).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_last(&self, sensor_id: &str) -> Result<Option<f64>, String> {
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        let key = format!("sensor:last:{}", sensor_id);
        let v: Option<f64> = conn.get(key).await.map_err(|e| e.to_string())?;
        Ok(v)
    }

    /// Flush all sensor:last:* to Raw (CBOR) - called on TTL/overflow.
    pub async fn flush_to_raw(&self, raw: &super::raw::RawStore) -> Result<usize, String> {
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        let keys: Vec<String> = redis::cmd("KEYS").arg("sensor:last:*").query_async(&mut conn).await.map_err(|e| e.to_string())?;
        let mut count = 0;
        for key in keys {
            let v: Option<f64> = conn.get(&key).await.map_err(|e| e.to_string())?;
            if let Some(value) = v {
                let sensor_id = key.trim_start_matches("sensor:last:").to_string();
                let point = super::raw::RawPoint {
                    sensor_id,
                    value,
                    ts_ms: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0),
                };
                raw.append(&point)?;
                count += 1;
            }
        }
        Ok(count)
    }
}

