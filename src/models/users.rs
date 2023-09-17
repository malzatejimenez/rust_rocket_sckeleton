use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Deserialize;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
