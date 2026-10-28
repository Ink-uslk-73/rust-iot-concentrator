//! src/storage/cache.rs - Redis hot-path cache (optional).

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
        let _: () = conn
            .set_ex(key, value, self.ttl_sec)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_last(&self, sensor_id: &str) -> Result<Option<f64>, String> {
        let mut conn = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;
        let key = format!("sensor:last:{}", sensor_id);
        let v: Option<f64> = conn.get(key).await.map_err(|e| e.to_string())?;
        Ok(v)
    }
}

