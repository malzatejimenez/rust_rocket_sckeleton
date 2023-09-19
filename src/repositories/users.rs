use crate::{
    models::{
        roles::{NewRole, Role},
        users::{NewUser, User},
        users_roles::{NewUserRole, UserRole},
    },
    repositories::roles::RoleRepository,
    schema::{
        roles::table as roles_table,
        users::{self, table as users_table},
        users_roles::{self, table as users_roles_table},
    },
};
use diesel::{prelude::*, PgConnection, QueryResult};

pub struct UserRepository;

impl UserRepository {
    pub fn find_by_username(c: &mut PgConnection, username: &str) -> QueryResult<User> {
        users_table.filter(users::username.eq(username)).first(c)
    }

    // Consulta los usuarios de la  base de datos incluyendo sus roles.
    pub fn find_with_roles(
        c: &mut PgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users_table.load(c)?;
        let result = users_roles_table
            .inner_join(roles_table)
            .load::<(UserRole, Role)>(c)?
            .grouped_by(&users);
        Ok(users.into_iter().zip(result).collect())
    }

    pub fn create(
        c: &mut PgConnection,
        new_user: NewUser,
        role_codes: Vec<String>,
    ) -> QueryResult<User> {
        // Insertar un nuevo usuario en la tabla de usuarios.
        let user = diesel::insert_into(users_table)
            .values(new_user)
            .get_result::<User>(c)?;

        // Recorrer la lista de códigos de roles proporcionados.
        for role_code in role_codes {
            // Intentar encontrar un rol existente por su código.
            let role: Role = {
                if let Ok(role) = RoleRepository::find_by_code(c, &role_code) {
                    role
                } else {
                    // Si el rol no existe, crear un nuevo rol.
                    let new_role = NewRole {
                        name: role_code.clone(),
                        code: role_code.clone(),
                    };
                    RoleRepository::create(c, new_role)?
                }
            };

            // Crear una nueva relación entre usuario y rol.
            let new_user_role = NewUserRole {
                user_id: user.id,
                role_id: role.id,
            };

            // Insertar la relación en la tabla de usuarios_roles.
            diesel::insert_into(users_roles_table)
                .values(new_user_role)
                .get_result::<UserRole>(c)?;
        }

        // Devolver el usuario creado.
        Ok(user)
    }

    // Método para eliminar un usuario por medio de su id.
    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        // Elimina registros en la tabla de relación users_roles que tienen el user_id igual a 'id'.
        // Esta tabla se utiliza para almacenar relaciones entre usuarios y roles.
        diesel::delete(users_roles_table.filter(users_roles::user_id.eq(id))).execute(c)?;

        // Elimina el registro del usuario en la tabla de usuarios (users_table) utilizando su 'id'.
        // Esto eliminará completamente al usuario de la base de datos.
        diesel::delete(users_table.find(id)).execute(c)
    }
}
