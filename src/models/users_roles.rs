use crate::schema::users_roles;
use diesel::{Insertable, Queryable};
use serde::Deserialize;

#[derive(Queryable)]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(User))]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}
