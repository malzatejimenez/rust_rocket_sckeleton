use crate::{
    models::{
        roles::{NewRole, Role},
        users::{NewUser, User},
        users_roles::{NewUserRole, UserRole},
    },
    repositories::roles::RoleRepository,
    schema::{users::table as users_table, users_roles::table as users_roles_table},
};
use diesel::{prelude::*, PgConnection, QueryResult};

pub struct UserRepository;

impl UserRepository {
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
}
