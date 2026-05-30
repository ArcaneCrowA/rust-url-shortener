use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestDTO {
    pub url: String,
}
