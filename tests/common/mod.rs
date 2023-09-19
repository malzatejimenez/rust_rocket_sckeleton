use std::process::Command;

use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{self, HeaderMap, HeaderValue},
    StatusCode,
};
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

fn get_logged_in_client(username: &str, role: &str) -> Client {
    // Se declara un string con el password del usuario
    let password = "1234";

    // Ejecuta el comando "cargo run --bin cli users create test_admin 1234 admin"
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg(password)
        .arg(role)
        .output()
        .unwrap();

    // println!("{:?}", output);

    // Se crea un cliente HTTP
    let client = Client::new();

    // Se obtiene el token del usuario administrador
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": username,
            "password": password,
        }))
        .send()
        .unwrap();

    // Se valida que el estatus de la respuesta sea OK
    assert_eq!(response.status(), StatusCode::OK);

    // Se obtiene el cuerpo de la respuesta
    let json: Value = response.json().unwrap();

    // Se valida que el cuerpo de la respuesta tenga un token
    assert!(json.get("token").is_some());

    // Se declara un header al cual se le agregará el token
    let mut headers = HeaderMap::new();

    // Se obtiene el token de la respuesta
    let token = json["token"].as_str().unwrap();

    // Se declara un header value al cual contendrá el token
    let header_token = HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap();

    // Se agrega header value al header map
    headers.insert(header::AUTHORIZATION, header_token);

    // Se crea un cliente HTTP con el header agregado
    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

// Función para crear un cliente HTTP con un usuario administrador autenticado
pub fn get_client_with_logged_in_admin() -> Client {
    get_logged_in_client("test_admin", "admin")
}

// Función para crear un cliente HTTP con un usuario viewer autenticado
pub fn get_client_with_logged_in_viewer() -> Client {
    get_logged_in_client("test_viewer", "viewer")
}
