extern crate cr8s;

use cr8s::{
    routes::{authorization, crates, rustaceans},
    DbConn,
};

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", authorization::routes())
        .mount("/", rustaceans::routes())
        .mount("/", crates::routes())
        .attach(DbConn::fairing())
        .launch()
        .await;
}
