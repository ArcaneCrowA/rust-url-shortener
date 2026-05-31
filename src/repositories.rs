use async_trait::async_trait;
use redis::AsyncCommands;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create_url(&self, code: &str, url: &str) -> Result<(), String>;
    async fn get_url(&self, code: &str) -> Result<String, String>;
}

pub struct RedisRepository {
    conn: redis::aio::MultiplexedConnection,
}

impl RedisRepository {
    pub async fn new() -> Result<Self, String> {
        let rdb = redis::Client::open("redis://127.0.0.1:6379")
            .map_err(|e| format!("Redis client error: {e}"))?;

        let conn = rdb
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| format!("Redis connection error: {e}"))?;

        Ok(Self { conn })
    }
}

#[async_trait]
impl Repository for RedisRepository {
    async fn create_url(&self, code: &str, url: &str) -> Result<(), String> {
        let mut conn = self.conn.clone();
        conn.set(code, url)
            .await
            .map_err(|e| format!("Redis set error: {e}"))
    }

    async fn get_url(&self, code: &str) -> Result<String, String> {
        let mut conn = self.conn.clone();
        conn.get(code)
            .await
            .map_err(|e| format!("Redis get error: {e}"))
    }
}
