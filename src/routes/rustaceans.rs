use crate::{
    models::rustaceans::{NewRustacean, Rustacean}, // Importa los modelos Rustacean y NewRustacean desde tu proyecto
    repositories::rustaceans::RustaceanRepository, // Importa el repositorio RustaceanRepository
    DbConn, // Importa la estructura DbConn que representa la conexión de base de datos
};
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value}, // Importa las utilidades para trabajar con JSON
};

// Ruta para obtener todos los Rustaceans
#[rocket::get("/rustaceans")]
async fn get_rustaceans(db: DbConn) -> Result<Value, Custom<Value>> {
    // Utiliza la conexión de base de datos 'db' para buscar múltiples Rustaceans (hasta 100)
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100) // Llama a la función find_multiple del repositorio
            .map(|rustaceans| json!(rustaceans)) // Convierte los resultados en JSON
            .map_err(|_| Custom(Status::InternalServerError, json!("error"))) // Maneja errores
    })
    .await
}

// Ruta para ver un Rustacean por su ID
#[rocket::get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    // Utiliza la conexión de base de datos 'db' para buscar un Rustacean por su ID
    db.run(move |c| {
        RustaceanRepository::find(c, id) // Llama a la función find del repositorio
            .map(|rustacean| json!(rustacean)) // Convierte el resultado en JSON
            .map_err(|_| Custom(Status::InternalServerError, json!("error"))) // Maneja errores
    })
    .await
}

// Ruta para crear un nuevo Rustacean
#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
async fn create_rustacean(
    new_rustacean: Json<NewRustacean>,
    db: DbConn,
) -> Result<Custom<Value>, Custom<Value>> {
    // Utiliza la conexión de base de datos 'db' para crear un nuevo Rustacean
    db.run(move |c| {
        let new_rustacean = new_rustacean.into_inner(); // Extrae los datos del JSON
        RustaceanRepository::create(c, new_rustacean) // Llama a la función create del repositorio
            // Devuelve un código de estado 201 (Created) junto con el Rustacean creado
            .map(|rustacean| Custom(Status::Created, json!(rustacean))) // Convierte el resultado en JSON
            .map_err(|_| Custom(Status::InternalServerError, json!("error"))) // Maneja errores
    })
    .await
}

// Ruta para actualizar un Rustacean por su ID
#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
async fn update_rustacean(
    id: i32,
    rustacean: Json<Rustacean>,
    db: DbConn,
) -> Result<Custom<Value>, Custom<Value>> {
    // Utiliza la conexión de base de datos 'db' para actualizar un Rustacean por su ID
    db.run(move |c| {
        let rustacean = rustacean.into_inner(); // Extrae los datos del JSON
        RustaceanRepository::update(c, id, rustacean) // Llama a la función update del repositorio
            // Devuelve un código de estado 200 (OK) junto con los detalles actualizados
            .map(|rustacean| Custom(Status::Ok, json!(rustacean))) // Convierte el resultado en JSON
            .map_err(|_| Custom(Status::InternalServerError, json!("error"))) // Maneja errores
    })
    .await
}

// Ruta para eliminar un Rustacean por su ID
#[rocket::delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
    // Utiliza la conexión de base de datos 'db' para eliminar un Rustacean por su ID
    db.run(move |c| {
        RustaceanRepository::delete(c, id) // Llama a la función delete del repositorio
            .map(|_| NoContent) // Devuelve una respuesta sin contenido si la eliminación es exitosa
            .map_err(|_| Custom(Status::InternalServerError, json!("error"))) // Maneja errores
    })
    .await
}

// Función que define todas las rutas
pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_rustaceans,
        view_rustacean,
        create_rustacean,
        update_rustacean,
        delete_rustacean,
    ]
}
