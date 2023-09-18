extern crate cr8s;

// Importamos las bibliotecas necesarias.
use clap::{Arg, Command};
use cr8s::commands::{create_user, delete_user, list_users};

fn main() {
    // se cargan las variables de entorno desde el archivo .env
    dotenv::dotenv().ok();

    // Creamos un objeto "Command" con el nombre "Cr8s" y una descripción general.
    let matches = Command::new("Cr8s")
        .about("Cr8s commands") // Descripción general del programa.
        .arg_required_else_help(true) // Mostrar ayuda si no se proporcionan argumentos.
        // Definimos un subcomando "users" para la gestión de usuarios.
        .subcommand(
            Command::new("users")
                .about("Users management") // Descripción del subcomando "users".
                .arg_required_else_help(true) // Mostrar ayuda si no se proporcionan argumentos.
                // Subcomando "create" para crear un usuario con roles.
                .subcommand(
                    Command::new("create")
                        .about("Create a user with multiple roles attached") // Descripción del subcomando.
                        .arg_required_else_help(true) // Mostrar ayuda si no se proporcionan argumentos.
                        // Argumentos para crear un usuario.
                        .arg(Arg::new("username").required(true)) // Nombre de usuario requerido.
                        .arg(Arg::new("password").required(true)) // Contraseña requerida.
                        .arg(
                            Arg::new("roles")
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ), // Roles separados por comas.
                )
                // Subcomando "list" para listar todos los usuarios disponibles.
                .subcommand(Command::new("list").about("List all available users"))
                // Subcomando "delete" para eliminar un usuario por ID.
                .subcommand(
                    Command::new("delete")
                        .about("Delete user by ID") // Descripción del subcomando.
                        .arg(Arg::new("id").required(true)), // ID del usuario a eliminar.
                ),
        )
        .get_matches(); // Obtener los argumentos proporcionados en la línea de comandos.

    // Comprobar qué subcomando se ha ejecutado.
    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => create_user(
                sub_matches
                    .get_one::<String>("username")
                    .unwrap()
                    .to_owned(),
                sub_matches
                    .get_one::<String>("password")
                    .unwrap()
                    .to_owned(),
                sub_matches
                    .get_many::<String>("roles")
                    .unwrap()
                    .map(|v| v.to_string())
                    .collect(),
            ), // Llamar a la función "create_user()" si se ejecuta el subcomando "create".
            Some(("list", _)) => list_users(), // Llamar a la función "list_users()" si se ejecuta el subcomando "list".
            Some(("delete", _)) => {
                delete_user(sub_matches.get_one::<i32>("id").unwrap().to_owned())
            } // Llamar a la función "delete_user()" si se ejecuta el subcomando "delete".
            _ => {}                            // Manejar otros casos.
        },
        _ => {} // Manejar otros casos.
    }
}
