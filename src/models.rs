use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestDTO {
    #[serde(deserialize_with = "validate_url")]
    pub url: String,
}

fn validate_url<'de, D: serde::Deserializer<'de>>(de: D) -> Result<String, D::Error> {
    let s = String::deserialize(de)?;

    let parsed = url::Url::parse(&s).map_err(serde::de::Error::custom)?;

    let scheme = parsed.scheme();
    if scheme != "http" && scheme != "https" {
        return Err(serde::de::Error::custom(format!(
            "URL must use http or https, got: {scheme}"
        )));
    }

    if parsed.host().is_none() {
        return Err(serde::de::Error::custom(format!(
            "URL must have a host: {s}"
        )));
    }

    Ok(s)
}
