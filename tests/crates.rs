use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;
use common::{
    create_test_crate, create_test_rustacean, delete_test_crate, delete_test_rustacean, APP_HOST,
};

#[test]
fn test_get_crates() {
    // SETUP ------------------------------
    // Creamos un cliente HTTP para realizar peticiones al servidor.
    let client = Client::new();

    // Creamos un Rustacean de prueba utilizando una función auxiliar.
    let rustacean = create_test_rustacean(&client);

    // Creamos un objeto JSON que representa un crate de prueba.
    let a_crate = create_test_crate(&client, &rustacean);
    let b_crate = create_test_crate(&client, &rustacean);

    // TEST -------------------------------

    // Realizamos una solicitud GET al servidor para obtener los crates.
    let response = client
        .get(format!("{}/crates", APP_HOST)) // Endpoint para crear crates
        .send() // Enviando la solicitud HTTP
        .unwrap(); // Manejo de errores

    // Verificamos que la respuesta sea un estado de "OK" (200).
    assert_eq!(response.status(), StatusCode::OK);

    // Analizamos el cuerpo de la respuesta JSON.
    let payload: Value = response.json().unwrap();

    // Verificamos que el cuerpo de la respuesta contenga los crates.
    assert!(payload.as_array().unwrap().contains(&a_crate));
    assert!(payload.as_array().unwrap().contains(&b_crate));

    // CLEANUP -----------------------------
    delete_test_crate(&client, a_crate); // Borramos el crate de prueba
    delete_test_crate(&client, b_crate); // Borramos el crate de prueba
    delete_test_rustacean(&client, rustacean); // Borramos el Rustacean de prueba
}

#[test]
fn test_create_crate() {
    // SETUP ------------------------------
    // Creamos un cliente HTTP para realizar peticiones al servidor.
    let client = Client::new();

    // Creamos un Rustacean de prueba utilizando una función auxiliar.
    let rustacean = create_test_rustacean(&client);

    // Creamos un objeto JSON que representa un crate de prueba.
    let mut a_crate = json!({
        "code": "foo",
        "name": "foo bar",
        "rustacean_id": rustacean["id"],
        "version": "0.1.0",
        "description": "foo bar baz",
    });

    // TEST -------------------------------

    // Realizamos una solicitud POST al servidor para crear el crate.
    let response = client
        .post(format!("{}/crates", APP_HOST)) // Endpoint para crear crates
        .json(&a_crate) // Enviamos el objeto JSON en la solicitud
        .send() // Enviando la solicitud HTTP
        .unwrap(); // Manejo de errores

    // Verificamos que la respuesta sea un estado de "CREATED" (201).
    assert_eq!(response.status(), StatusCode::CREATED);

    // Analizamos el cuerpo de la respuesta JSON.
    let payload: Value = response.json().unwrap();
    // Actualizamos el objeto "a_crate" con el "id" y "created_at" generados por el servidor.
    a_crate["id"] = payload["id"].clone();
    a_crate["created_at"] = payload["created_at"].clone();

    // Comparamos el objeto "a_crate" con el payload recibido para asegurarnos de que sean iguales.
    assert_eq!(a_crate, payload);

    // CLEANUP -----------------------------
    delete_test_crate(&client, a_crate); // Borramos el crate de prueba
    delete_test_rustacean(&client, rustacean); // Borramos el Rustacean de prueba
}

#[test]
fn test_view_crate() {
    // SETUP ------------------------------

    // Creamos un cliente HTTP para realizar peticiones al servidor.
    let client = Client::new();

    // Creamos un Rustacean de prueba utilizando una función auxiliar.
    let rustacean = create_test_rustacean(&client);

    let a_crate = create_test_crate(&client, &rustacean); // Creamos el crate de prueba

    // TEST -------------------------------

    // Realizamos una solicitud GET al servidor para obtener el crate a través de su id.
    let response = client
        .get(format!("{}/crates/{}", APP_HOST, a_crate["id"])) // Endpoint para crear crates
        .send() // Enviando la solicitud HTTP
        .unwrap(); // Manejo de errores

    // Verificamos que la respuesta sea un estado de "OK" (200).
    assert_eq!(response.status(), StatusCode::OK);

    // Analizamos el cuerpo de la respuesta JSON.
    let payload: Value = response.json().unwrap();

    // Comparamos el objeto "a_crate" con el payload recibido para asegurarnos de que sean iguales.
    assert_eq!(a_crate, payload);

    // CLEANUP -----------------------------
    delete_test_crate(&client, a_crate); // Borramos el crate de prueba
    delete_test_rustacean(&client, rustacean); // Borramos el Rustacean de prueba
}

#[test]
fn test_update_crate() {
    // SETUP ------------------------------

    // Creamos un cliente HTTP para realizar peticiones al servidor.
    let client = Client::new();

    // Creamos un Rustacean de prueba utilizando una función auxiliar.
    let rustacean = create_test_rustacean(&client);

    let a_crate = create_test_crate(&client, &rustacean); // Creamos el crate de prueba

    // TEST -------------------------------

    let crate_to_edit = json!({
        "id": a_crate["id"],
        "code": "foozie",
        "name": "foozie bar",
        "rustacean_id": rustacean["id"],
        "version": "0.1.1",
        "description": "foozie bar baz",
        "created_at": a_crate["created_at"],
    });

    // Realizamos una solicitud PUT al servidor para obtener el crate a través de su id.
    let response = client
        .put(format!("{}/crates/{}", APP_HOST, a_crate["id"])) // Endpoint para crear crates
        .json(&crate_to_edit) // Enviamos el objeto JSON en la solicitud
        .send() // Enviando la solicitud HTTP
        .unwrap(); // Manejo de errores

    // Verificamos que la respuesta sea un estado de "OK" (200).
    assert_eq!(response.status(), StatusCode::OK);

    // Analizamos el cuerpo de la respuesta JSON.
    let payload: Value = response.json().unwrap();

    // Comparamos el objeto "a_crate" con el payload recibido para asegurarnos de que sean iguales.
    assert_eq!(crate_to_edit, payload);

    // CLEANUP -----------------------------
    delete_test_crate(&client, a_crate); // Borramos el crate de prueba
    delete_test_rustacean(&client, rustacean); // Borramos el Rustacean de prueba
}

#[test]
fn test_delete_crate() {
    // SETUP ------------------------------

    // Creamos un cliente HTTP para realizar peticiones al servidor.
    let client = Client::new();

    // Creamos un Rustacean de prueba utilizando una función auxiliar.
    let rustacean = create_test_rustacean(&client);

    let a_crate = create_test_crate(&client, &rustacean); // Creamos el crate de prueba

    // TEST -------------------------------

    // Realizamos una solicitud DELETE al servidor para eliminar el crate a través de su id.
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, a_crate["id"])) // Endpoint para crear crates
        .send() // Enviando la solicitud HTTP
        .unwrap(); // Manejo de errores

    // Verificamos que la respuesta sea un estado de "NO_CONTENT" (204).
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // CLEANUP -----------------------------
    delete_test_rustacean(&client, rustacean); // Borramos el Rustacean de prueba
}
