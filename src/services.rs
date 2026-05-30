use std::sync::Arc;

const PREFIX: &str = "url:";

use uuid::Uuid;

use crate::repositories::Repository;

pub struct Service {
    repo: Arc<dyn Repository>,
}

impl Service {
    pub fn new(repo: Arc<dyn Repository>) -> Self {
        Self { repo }
    }

    pub async fn create_url(&self, url: &str) -> Result<String, String> {
        let v4 = Uuid::new_v4();
        let code = &v4.to_string()[..6];
        let prefix_code = PREFIX.to_owned() + code;

        if let Err(msg) = self.repo.create_url(&prefix_code, url).await {
            Err("couldn't create url: ".to_owned() + &msg)
        } else {
            Ok("localhost:3000/".to_owned() + code)
        }
    }

    pub async fn get_url(&self, code: &str) -> Result<String, String> {
        let prefix_code = PREFIX.to_owned() + code;
        self.repo.get_url(&prefix_code).await
    }
}
