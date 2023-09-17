pub mod crates;
pub mod rustaceans;

use rocket::{http::Status, response::status::Custom};
use serde_json::{json, Value};

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}
