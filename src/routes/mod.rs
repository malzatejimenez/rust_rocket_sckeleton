pub mod authorization;
pub mod crates;
pub mod rustaceans;

use crate::{models::users::User, repositories::users::UserRepository};
use rocket::{
    http::Status, request::FromRequest, request::Outcome, response::status::Custom, Request,
};
use rocket_db_pools::{
    deadpool_redis::{redis::AsyncCommands, Pool},
    Connection, Database,
};
use serde_json::{json, Value};

// Define una función para manejar errores
pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

// Define una estructura que representa la conexión a la base de datos PostgreSQL
// Utiliza la macro rocket_sync_db_pools::database para gestionar la conexión
#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(diesel::PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(Pool);

// Implementa la interfaz FromRequest para la conexión a la base de datos
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer <token>
        // Obtener el token
        let session_header = request
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        // Validar el token
        if let Some(session_value) = session_header {
            // Obtener la conexión a la base de datos del cache (Redis)
            let mut cache = request
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Cannot get redis connection");

            // Obtener la conexión a la base de datos de Postgres
            let db = request
                .guard::<DbConn>()
                .await
                .expect("Cannot get postgres connection");

            // Obtener el id de la sesión
            let result = cache
                .get::<_, i32>(format!("sessions/{}", session_value[1]))
                .await;

            // Validar el id de la sesión
            if let Ok(session_user_id) = result {
                return match db
                    .run(move |c| UserRepository::find(c, session_user_id))
                    .await
                {
                    Ok(user) => Outcome::Success(user),
                    Err(_) => Outcome::Failure((Status::Unauthorized, ())),
                };
            };
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
