use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

// Función para crear un Rustacean utilizando un cliente HTTP
pub fn create_test_rustacean(client: &Client) -> Value {
    // Crea un objeto JSON que representa un Rustacean con nombre y correo electrónico
    let rustacean = json!({
        "name": "Foo bar",
        "email": "foo@bar.com",
    });

    // Realiza una solicitud POST al servidor local de Rocket para crear el Rustacean
    let response = client
        .post(format!("{}/rustaceans", APP_HOST)) // URL de la API
        .json(&rustacean) // Envía el objeto JSON en la solicitud
        .send() // Envía la solicitud y obtiene la respuesta
        .unwrap(); // Maneja cualquier error que pueda ocurrir

    // Verifica que la respuesta tenga un código de estado HTTP 201 (CREATED)
    assert_eq!(response.status(), StatusCode::CREATED);

    // Deserializa la respuesta JSON y la devuelve como un objeto Value
    response.json().unwrap()
}

// funcion para elminar un rustacean utilizando un cliente HTTP
pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    // Enviar una solicitud DELETE para eliminar el Rustacean creado
    let response = client
        .delete(format!(
            "{}/rustaceans/{}",
            APP_HOST,
            rustacean["id"] // Utilizar el "id" del Rustacean creado en la URL
        ))
        .send()
        .unwrap();

    // Validar que el código de estado de la respuesta sea NO CONTENT (204)
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

// funcion para elminar un crate utilizando un cliente HTTP
pub fn delete_test_crate(client: &Client, a_crate: Value) {
    // Enviar una solicitud DELETE para eliminar el Rustacean creado
    let response = client
        .delete(format!(
            "{}/crates/{}",
            APP_HOST,
            a_crate["id"] // Utilizar el "id" del Rustacean creado en la URL
        ))
        .send()
        .unwrap();

    // Validar que el código de estado de la respuesta sea NO CONTENT (204)
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

// Función para crear un crate utilizando un cliente HTTP
pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    // Crea un objeto JSON que representa un Rustacean con nombre y correo electrónico
    let a_crate = json!({
        "code": "foo",
        "name": "foo bar",
        "rustacean_id": rustacean["id"],
        "version": "0.1.0",
        "description": "foo bar baz",
    });

    // Realiza una solicitud POST al servidor local de Rocket para crear el crate
    let response = client
        .post(format!("{}/crates", APP_HOST)) // URL de la API
        .json(&a_crate) // Envía el objeto JSON en la solicitud
        .send() // Envía la solicitud y obtiene la respuesta
        .unwrap(); // Maneja cualquier error que pueda ocurrir

    // Verifica que la respuesta tenga un código de estado HTTP 201 (CREATED)
    assert_eq!(response.status(), StatusCode::CREATED);

    // Deserializa la respuesta JSON y la devuelve como un objeto Value
    response.json().unwrap()
}
