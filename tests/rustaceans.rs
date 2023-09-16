use reqwest::{blocking::Client, StatusCode}; // Importa las bibliotecas necesarias
use rocket::serde::json::{serde_json::json, Value}; // Importa las bibliotecas necesarias de Rocket

// Función para crear un Rustacean utilizando un cliente HTTP
fn create_test_rustacean(client: &Client) -> Value {
    // Crea un objeto JSON que representa un Rustacean con nombre y correo electrónico
    let rustacean = json!({
        "name": "Foo bar",
        "email": "foo@bar.com",
    });

    // Realiza una solicitud POST al servidor local de Rocket para crear el Rustacean
    let response = client
        .post("http://127.0.0.1:8000/rustaceans") // URL de la API
        .json(&rustacean) // Envía el objeto JSON en la solicitud
        .send() // Envía la solicitud y obtiene la respuesta
        .unwrap(); // Maneja cualquier error que pueda ocurrir

    // Verifica que la respuesta tenga un código de estado HTTP 201 (CREATED)
    assert_eq!(response.status(), StatusCode::CREATED);

    // Deserializa la respuesta JSON y la devuelve como un objeto Value
    response.json().unwrap()
}

// funcion para elminar un rustacean utilizando un cliente HTTP
fn delete_test_rustacean(client: &Client, rustacean: Value) {
    // Enviar una solicitud DELETE para eliminar el Rustacean creado
    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"] // Utilizar el "id" del Rustacean creado en la URL
        ))
        .send()
        .unwrap();

    // Validar que el código de estado de la respuesta sea NO CONTENT (204)
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_rustaceans() {
    // SETUP ----------------------------

    // Definir un cliente HTTP para realizar la prueba
    let client = Client::new();

    let rustacean1 = create_test_rustacean(&client); // Crea un Rustacean
    let rustacean2 = create_test_rustacean(&client); // Crea un Rustacean

    // TEST ----------------------------

    // Enviar una solicitud GET para obtener la lista de Rustaceans desde la API
    let response = client
        .get("http://127.0.0.1:8000/rustaceans") // URL de la API para obtener Rustaceans
        .send() // Enviar la solicitud y obtener la respuesta
        .unwrap(); // Manejar cualquier error que pueda ocurrir

    // Validar que el código de estado de la respuesta sea OK (200)
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap(); // Deserializar la respuesta JSON

    assert!(json.as_array().unwrap().contains(&rustacean1)); // Validar que la lista de Rustaceans contenga el Rustacean 1
    assert!(json.as_array().unwrap().contains(&rustacean2)); // Validar que la lista de Rustaceans contenga el Rustacean 2

    // CLEANUP ----------------------------
    delete_test_rustacean(&client, rustacean1); // Eliminar el Rustacean 1
    delete_test_rustacean(&client, rustacean2); // Eliminar el Rustacean 2
}

#[test]
fn test_create_rustacean() {
    // SETUP-----------------------------

    // Crea un cliente HTTP para realizar la prueba
    let client = Client::new();

    // Crea un objeto JSON que representa un Rustacean con nombre y correo electrónico
    let mut rustacean = json!({
        "name": "Foo bar",
        "email": "foo@bar.com",
    });

    // TEST ----------------------------

    // Realiza una solicitud POST al servidor local de Rocket para crear el Rustacean
    let response = client
        .post("http://127.0.0.1:8000/rustaceans") // URL de la API
        .json(&rustacean) // Envía el objeto JSON en la solicitud
        .send() // Envía la solicitud y obtiene la respuesta
        .unwrap(); // Maneja cualquier error que pueda ocurrir

    // Verifica que la respuesta tenga un código de estado HTTP 201 (CREATED)
    assert_eq!(response.status(), StatusCode::CREATED);

    // Deserializa la respuesta JSON en un objeto Value
    let payload: Value = response.json().unwrap();

    // Actualiza el objeto rustacean con el "id" y "created_at" de la respuesta
    rustacean["id"] = payload["id"].clone();
    rustacean["created_at"] = payload["created_at"].clone();

    // Verifica que la respuesta coincida con el objeto actualizado rustacean
    assert_eq!(payload, rustacean);

    // CLEANUP ----------------------------
    delete_test_rustacean(&client, rustacean); // Eliminar el Rustacean creado
}

#[test]
fn test_view_rustacean() {
    // SETUP -------------------------------

    // definiendo el cliente
    let client = Client::new();

    // creando un rustacean
    let rustacean_created: Value = create_test_rustacean(&client);

    // TEST -------------------------------

    // consultando el rustacean creado
    let response = client
        .get(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean_created["id"]
        ))
        .send()
        .unwrap();

    // validando que el status sea OK
    assert_eq!(response.status(), StatusCode::OK);

    // obteniendo el rustacean consultado
    let rustacean_viewed: Value = response.json().unwrap();

    // validando que el rustacean consultado sea el mismo que el creado inicialmente
    assert_eq!(rustacean_created, rustacean_viewed);

    // CLEANUP -------------------------------
    delete_test_rustacean(&client, rustacean_created); // Eliminar el Rustacean creado
}

#[test]
fn test_update_rustacean() {
    // SETUP -------------------------------

    // Definir un cliente HTTP para realizar la prueba
    let client = Client::new();

    // Crear un Rustacean inicial utilizando la función create_test_rustacean
    let rustacean_created: Value = create_test_rustacean(&client);

    // Crear una versión actualizada del Rustacean con los mismos detalles, incluido el "id"
    let rustacean_to_update = json!({
        "id": rustacean_created["id"],       // Mismo "id" que el Rustacean creado
        "name": "Fooz",
        "email": "fooz@bar.com",
        "created_at": rustacean_created["created_at"],  // Mismo "created_at"
    });

    // TEST -------------------------------

    // Enviar una solicitud PUT para actualizar el Rustacean creado
    let response = client
        .put(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean_created["id"] // Utilizar el "id" del Rustacean creado en la URL
        ))
        .json(&rustacean_to_update) // Envía la versión actualizada del Rustacean
        .send()
        .unwrap();

    // Validar que el código de estado de la respuesta sea OK (200)
    assert_eq!(response.status(), StatusCode::OK);

    // Obtener el Rustacean actualizado desde la respuesta
    let rustacean_updated: Value = response.json().unwrap();

    // Validar que el Rustacean creado sea igual al Rustacean actualizado
    assert_eq!(rustacean_to_update, rustacean_updated);

    // CLEANUP -------------------------------
    delete_test_rustacean(&client, rustacean_created); // Eliminar el Rustacean creado
}

#[test]
fn test_delete_rustacean() {
    // SETUP -------------------------------

    // Definir un cliente HTTP para realizar la prueba
    let client = Client::new();

    // Crear un Rustacean utilizando la función create_test_rustacean para luego eliminarlo
    let rustacean_created: Value = create_test_rustacean(&client);

    // TEST -------------------------------

    // Enviar una solicitud DELETE para eliminar el Rustacean creado
    let response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean_created["id"] // Utilizar el "id" del Rustacean creado en la URL
        ))
        .send()
        .unwrap();

    // Validar que el código de estado de la respuesta sea NO CONTENT (204)
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
