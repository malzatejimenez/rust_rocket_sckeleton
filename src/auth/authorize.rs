use super::Credentials;
use crate::models::users::User;
use argon2::{
    self,
    password_hash::{self, rand_core::OsRng, SaltString},
    Argon2,
    PasswordHash,
    PasswordHasher, // Importamos PasswordHasher para el hash de contraseñas
    PasswordVerifier,
};
use rand::{distributions::Alphanumeric, Rng};

pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    // Generamos una cadena de sal aleatoria
    let salt = SaltString::generate(&mut OsRng);

    // Creamos una instancia de Argon2 con la configuración predeterminada
    let argon = argon2::Argon2::default();

    // Hash de la contraseña y manejo de errores
    let password_hash = argon.hash_password(password.as_bytes(), &salt)?; // Convertimos errores en cadenas de texto

    // Devolvemos el hash de la contraseña como una cadena
    Ok(password_hash.to_string())
}

pub fn authorize_user(
    user: &User,
    credentials: &Credentials,
) -> Result<String, password_hash::Error> {
    let db_hash = PasswordHash::new(&user.password)?;
    let argon = Argon2::default();
    argon.verify_password(credentials.password.as_bytes(), &db_hash)?;

    Ok(rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect())
}
