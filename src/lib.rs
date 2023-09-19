// Importa los módulos necesarios para tu aplicación
mod auth;
pub mod commands; // Módulo para definir comandos o acciones
mod models; // Módulo para definir modelos de datos
mod repositories; // Módulo para definir repositorios de datos
pub mod routes; // Módulo para definir rutas o endpoints
mod schema; // Módulo para definir la estructura de la base de datos

use rocket_db_pools::{deadpool_redis, Database};

// Define una estructura que representa la conexión a la base de datos PostgreSQL
// Utiliza la macro rocket_sync_db_pools::database para gestionar la conexión
#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(diesel::PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);
