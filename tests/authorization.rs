use reqwest::{blocking::Client, StatusCode}; // Importamos las bibliotecas necesarias para hacer solicitudes HTTP y manejar códigos de estado.
use serde_json::{json, Value}; // Importamos las bibliotecas necesarias para trabajar con JSON.
use std::process::Command; // Importamos la biblioteca para ejecutar comandos en el sistema.

pub mod common; // Importamos el módulo "common" que contiene constantes.
use common::APP_HOST; // Importamos la constante APP_HOST del módulo "common".

#[test]
fn test_login() {
    // Ejecutamos un comando de Rust para crear un usuario de prueba.
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap(); // Capturamos la salida y la imprimimos.

    println!("{:?}", output);

    let client = Client::new(); // Creamos una instancia de cliente HTTP con reqwest.

    // Creamos un objeto JSON que representa las credenciales de acceso.
    let credentials = json!({
        "username": "test_admin",
        "password": "1234",
    });

    // Realizamos una solicitud POST al servidor para iniciar sesión.
    let response = client
        .post(format!("{}/login", APP_HOST)) // Especificamos la URL del endpoint para iniciar sesión.
        .json(&credentials) // Enviamos el objeto JSON en la solicitud.
        .send() // Enviando la solicitud HTTP.
        .unwrap(); // Manejo de errores.

    assert_eq!(response.status(), StatusCode::OK); // Comprobamos que la respuesta es OK.

    let json: Value = response.json().unwrap(); // Parseamos la respuesta JSON.

    // Comprobamos que el token está presente en la respuesta.
    assert!(json.get("token").is_some());

    // Comprobamos que la longitud del token es de 128 caracteres.
    assert_eq!(
        json["token"]
            .as_str()
            .expect("El token no está presente en la respuesta")
            .len(),
        128
    );

    // Creamos un objeto JSON que representa las credenciales de acceso erróneas (contraseña incorrecta).
    let credentials = json!({
        "username": "test_admin",
        "password": "12345",
    });

    // Realizamos una solicitud POST al servidor para iniciar sesión pero con credenciales erróneas.
    let response = client
        .post(format!("{}/login", APP_HOST)) // Especificamos la URL del endpoint para iniciar sesión.
        .json(&credentials) // Enviamos el objeto JSON en la solicitud.
        .send() // Enviando la solicitud HTTP.
        .unwrap(); // Manejo de errores.

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED); // Comprobamos que la respuesta es Unauthorized.

    // Creamos un objeto JSON que representa las credenciales de acceso erróneas (usuario incorrecto).
    let credentials = json!({
        "username": "test_admin_wrong",
        "password": "1234",
    });

    // Realizamos una solicitud POST al servidor para iniciar sesión pero con credenciales erróneas.
    let response = client
        .post(format!("{}/login", APP_HOST)) // Especificamos la URL del endpoint para iniciar sesión.
        .json(&credentials) // Enviamos el objeto JSON en la solicitud.
        .send() // Enviando la solicitud HTTP.
        .unwrap(); // Manejo de errores.

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED); // Comprobamos que la respuesta es Unauthorized.
}
