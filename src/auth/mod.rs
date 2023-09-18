pub mod authorize;

#[derive(Debug, serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}
