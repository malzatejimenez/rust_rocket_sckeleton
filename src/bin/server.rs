extern crate cr8s;

use cr8s::{
    routes::{crates, rustaceans},
    DbConn,
};

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rustaceans::routes())
        .mount("/", crates::routes())
        .attach(DbConn::fairing())
        .launch()
        .await;
}
