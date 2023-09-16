#[macro_use]
extern crate rocket;

mod models;
mod repositories;
mod routes;
mod schema;

use routes::{crates, rustaceans};

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(diesel::PgConnection);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rustaceans::routes())
        .mount("/", crates::routes())
        .attach(DbConn::fairing())
        .launch()
        .await;
}
