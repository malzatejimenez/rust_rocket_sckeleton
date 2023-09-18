use crate::{
    auth::authorize::hash_password,
    models::users::NewUser,
    repositories::{roles::RoleRepository, users::UserRepository},
};
use diesel::{pg::PgConnection, prelude::*};

// Función para cargar la conexión a la base de datos desde la variable de entorno
fn load_db_connection() -> PgConnection {
    let database_url =
        std::env::var("DATABASE_URL").expect("Cannot find database url from environment variables");

    // Establece la conexión a la base de datos PostgreSQL
    PgConnection::establish(&database_url).expect("Cannot connect to database")
}

// Función para crear un nuevo usuario
pub fn create_user(username: String, password: String, roles: Vec<String>) -> () {
    // Carga la conexión a la base de datos
    let mut c = load_db_connection();

    // Se genera el hash del password
    let password_hash = hash_password(&password).unwrap();

    // Crea un nuevo usuario con los datos proporcionados
    let new_user = NewUser {
        username,
        password: password_hash,
    };

    // Crea el usuario en la base de datos y obtiene el resultado
    let user = UserRepository::create(&mut c, new_user, roles).unwrap();

    // Imprime un mensaje indicando que el usuario se ha creado con éxito
    println!("User created {:?}", user);

    // Busca los roles asociados a este usuario y obtiene el resultado
    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();

    // Imprime los roles asociados a este usuario
    println!("Role: {:?}", roles);
}

// Función para listar usuarios
pub fn list_users() -> () {
    // Carga la conexión a la base de datos
    let mut c = load_db_connection();

    // Se obtienen todos los usuarios de la base de datos con sus roles
    let users = UserRepository::find_with_roles(&mut c).unwrap();

    // Se imprimen los usuarios
    for user in users {
        println!("User: {:?}", user);
    }
}

// Función para eliminar un usuario por su ID
pub fn delete_user(id: i32) -> () {
    // Carga la conexión a la base de datos
    let mut c = load_db_connection();

    // Elimina el usuario
    UserRepository::delete(&mut c, id).unwrap();
}
