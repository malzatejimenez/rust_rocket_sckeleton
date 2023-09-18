use crate::{
    models::{
        roles::{NewRole, Role},
        users::User,
        users_roles::UserRole,
    },
    schema::roles::{self, table as roles_table},
};
use diesel::{prelude::*, BelongingToDsl, PgConnection, QueryResult};

pub struct RoleRepository;

impl RoleRepository {
    // Encuentra un rol por su código
    pub fn find_by_code(c: &mut PgConnection, code: &str) -> QueryResult<Role> {
        roles_table.filter(roles::code.eq(code)).first(c)
    }

    // Encuentra roles por sus IDs
    pub fn find_by_ids(c: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles_table.filter(roles::id.eq_any(ids)).get_results(c)
    }

    // Encuentra roles asociados a un usuario
    pub fn find_by_user(c: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        // Obtén los roles asociados al usuario
        let user_roles = UserRole::belonging_to(&user).get_results(c)?;

        // Extrae los IDs de los roles
        let role_ids = user_roles
            .iter()
            .map(|ur: &UserRole| ur.role_id)
            .collect::<Vec<i32>>();

        // Con los IDs de roles, obtén la información completa de los roles
        Self::find_by_ids(c, role_ids)
    }

    // Crea un nuevo rol
    pub fn create(c: &mut PgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles_table)
            .values(new_role)
            .get_result::<Role>(c)
    }
}
