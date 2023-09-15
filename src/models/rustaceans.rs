use crate::schema::rustaceans;
use chrono::NaiveDateTime;
use diesel::{query_builder::AsChangeset, Insertable, Queryable};

#[derive(Queryable, AsChangeset)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}
