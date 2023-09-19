use super::{server_error, DbConn, EditorUser};
use crate::{
    models::crates::{Crate, NewCrate},
    repositories::crates::CrateRepository, // Importa el repositorio CrateRepository
};
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    routes,
    serde::json::{serde_json::json, Json, Value}, // Importa las utilidades para trabajar con JSON
};

// Ruta para obtener todos los Crates
#[rocket::get("/crates")]
async fn get_crates(db: DbConn, _user: EditorUser) -> Result<Value, Custom<Value>> {
    // Utiliza la conexión de base de datos 'db' para buscar múltiples Crates (hasta 100)
    db.run(|c| {
        CrateRepository::find_multiple(c, 100) // Llama a la función find_multiple del repositorio
            .map(|crates| json!(crates)) // Convierte los resultados en JSON
            .map_err(|e| server_error(e.into())) // Maneja errores
    })
    .await
}

// Ruta para ver un Crate por su ID
#[rocket::get("/crates/<id>")]
async fn view_crate(id: i32, db: DbConn, _user: EditorUser) -> Result<Value, Custom<Value>> {
    // Utiliza la conexión de base de datos 'db' para buscar un Crate por su ID
    db.run(move |c| {
        CrateRepository::find(c, id) // Llama a la función find del repositorio
            .map(|a_crate| json!(a_crate)) // Convierte el resultado en JSON
            .map_err(|e| server_error(e.into())) // Maneja errores
    })
    .await
}

// Ruta para crear un nuevo Crate
#[rocket::post("/crates", format = "json", data = "<new_crate>")]
async fn create_crate(
    new_crate: Json<NewCrate>,
    db: DbConn,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    // Utiliza la conexión de base de datos 'db' para crear un nuevo Crate
    db.run(move |c| {
        let new_crate = new_crate.into_inner(); // Extrae los datos del JSON
        CrateRepository::create(c, new_crate) // Llama a la función create del repositorio
            // Devuelve un código de estado 201 (Created) junto con el Crate creado
            .map(|a_crate| Custom(Status::Created, json!(a_crate))) // Convierte el resultado en JSON
            .map_err(|e| server_error(e.into())) // Maneja errores
    })
    .await
}

// Ruta para actualizar un Crate por su ID
#[rocket::put("/crates/<id>", format = "json", data = "<a_crate>")]
async fn update_crate(
    id: i32,
    a_crate: Json<Crate>,
    db: DbConn,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    // Utiliza la conexión de base de datos 'db' para actualizar un Crate por su ID
    db.run(move |c| {
        let a_crate = a_crate.into_inner(); // Extrae los datos del JSON
        CrateRepository::update(c, id, a_crate) // Llama a la función update del repositorio
            // Devuelve un código de estado 200 (OK) junto con los detalles actualizados
            .map(|a_crate| Custom(Status::Ok, json!(a_crate))) // Convierte el resultado en JSON
            .map_err(|e| server_error(e.into())) // Maneja errores
    })
    .await
}

// Ruta para eliminar un Crate por su ID
#[rocket::delete("/crates/<id>")]
async fn delete_crate(id: i32, db: DbConn, _user: EditorUser) -> Result<NoContent, Custom<Value>> {
    // Utiliza la conexión de base de datos 'db' para eliminar un Crate por su ID
    db.run(move |c| {
        CrateRepository::delete(c, id) // Llama a la función delete del repositorio
            .map(|_| NoContent) // Devuelve una respuesta sin contenido si la eliminación es exitosa
            .map_err(|e| server_error(e.into())) // Maneja errores
    })
    .await
}

// Función que define todas las rutas
pub fn routes() -> Vec<rocket::Route> {
    routes![
        get_crates,
        view_crate,
        create_crate,
        update_crate,
        delete_crate,
    ]
}
