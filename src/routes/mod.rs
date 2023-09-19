pub mod authorization;
pub mod crates;
pub mod rustaceans;

use rocket::{http::Status, response::status::Custom};
use serde_json::{json, Value};

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

use rocket_db_pools::{deadpool_redis, Database};

// Define una estructura que representa la conexión a la base de datos PostgreSQL
// Utiliza la macro rocket_sync_db_pools::database para gestionar la conexión
#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(diesel::PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);
