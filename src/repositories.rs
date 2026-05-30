use async_trait::async_trait;
use redis::Commands;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create_url(&self, code: &str, url: &str) -> Result<(), String>;
    async fn get_url(&self, code: &str) -> Result<String, String>;
}

pub struct RedisRepository {
    rdb: redis::Client,
}

impl RedisRepository {
    pub fn new() -> Self {
        Self {
            rdb: redis::Client::open("redis://127.0.0.1:6379").unwrap(),
        }
    }
}

#[async_trait]
impl Repository for RedisRepository {
    async fn create_url(&self, code: &str, url: &str) -> Result<(), String> {
        if let Ok(res) = self.rdb.clone().set(code, url) {
            let _: String = res;
            Ok(())
        } else {
            Err("Error setting value".to_owned())
        }
    }

    async fn get_url(&self, code: &str) -> Result<String, String> {
        if let Ok(res) = self.rdb.clone().get(code) {
            let res: String = res;
            Ok(res)
        } else {
            Err("Error getting value".to_owned())
        }
    }
}
