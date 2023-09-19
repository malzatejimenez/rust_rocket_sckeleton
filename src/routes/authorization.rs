use super::server_error;
use crate::{
    auth::{authorize::authorize_user, Credentials},
    repositories::users::UserRepository,
    CacheConn, DbConn,
};
use rocket::{
    http::Status,
    response::status::Custom,
    routes,
    serde::json::{serde_json::json, Json, Value},
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

#[rocket::post("/login", format = "json", data = "<credentials>")]
async fn login(
    credentials: Json<Credentials>,
    db: DbConn,
    mut cache: Connection<CacheConn>,
) -> Result<Value, Custom<Value>> {
    let username = credentials.username.clone();
    let user = db
        .run(move |c| {
            UserRepository::find_by_username(c, &username).map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    Custom(Status::Unauthorized, json!("Wrong credentials"))
                }
                _ => server_error(e.into()),
            })
        })
        .await?;

    let session_id = authorize_user(&user, &credentials)
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    cache
        .set_ex::<_, _, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map(|_| json!({"token": session_id}))
        .map_err(|e| server_error(e.into()))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login]
}
