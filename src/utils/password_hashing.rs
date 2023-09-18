use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    PasswordHasher,
    PasswordVerifier, // Importamos PasswordHasher para el hash de contraseñas
};

pub fn hash_password(password: &str) -> Result<String, String> {
    // Generamos una cadena de sal aleatoria
    let salt = SaltString::generate(&mut OsRng);

    // Creamos una instancia de Argon2 con la configuración predeterminada
    let argon = argon2::Argon2::default();

    // Hash de la contraseña y manejo de errores
    let password_hash = argon
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?; // Convertimos errores en cadenas de texto

    // Devolvemos el hash de la contraseña como una cadena
    Ok(password_hash.to_string())
}

// Función para verificar una contraseña con su hash
pub fn _verify_password(password: &str, hashed_password: &str) -> bool {
    let password_bytes = password.as_bytes();

    // Convierte el hash de contraseña en un tipo PasswordHash
    let hash = argon2::PasswordHash::new(hashed_password).expect("Error al crear PasswordHash");

    // Utiliza la función de verificación
    argon2::Argon2::default()
        .verify_password(password_bytes, &hash)
        .is_ok()
}

// Importamos las bibliotecas necesarias para las pruebas
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        // Define una contraseña de prueba
        let password = "contraseña_secreta";

        // Llama a la función hash_password
        let result = hash_password(password);

        // Verifica que la función devuelva un resultado Ok
        assert!(result.is_ok());

        // Obtén el hash de contraseña resultante
        let hashed_password = result.unwrap();

        // Verifica que el hash de contraseña no esté vacío
        assert!(!hashed_password.is_empty());
    }

    #[test]
    fn test_verify_password() {
        // Define una contraseña de prueba
        let password = "contraseña_secreta";

        // Genera una cadena de sal aleatoria
        let salt = SaltString::generate(&mut OsRng);

        // Crea una instancia de Argon2 con la configuración predeterminada
        let argon = argon2::Argon2::default();

        // Hash de la contraseña
        let password_hash = argon
            .hash_password(password.as_bytes(), &salt)
            .expect("Error al generar el hash de contraseña");

        // Verifica que la contraseña coincida con su hash
        let result = _verify_password(password, &password_hash.to_string());

        // Debe ser true, ya que la contraseña y el hash coinciden
        assert!(result);
    }
}
