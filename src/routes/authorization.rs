use super::server_error;
use crate::{
    auth::{authorize::authorize_user, Credentials},
    repositories::users::UserRepository,
    DbConn,
};

use rocket::{
    response::status::Custom,
    routes,
    serde::json::{serde_json::json, Json, Value},
};

#[rocket::post("/login", format = "json", data = "<credentials>")]
async fn login(credentials: Json<Credentials>, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::find_by_username(c, &credentials.username)
            .map(|user| {
                if let Ok(token) = authorize_user(&user, &credentials) {
                    return json!(token);
                }
                json!("Unauthorized")
            })
            .map_err(|e| server_error(e.into()))
    })
    .await
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login]
}
