use crate::{
    models::{roles::Role, users::User},
    schema::users_roles,
};
use diesel::{
    prelude::{Associations, Identifiable},
    Insertable, Queryable,
};
use serde::Deserialize;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(User))]
#[diesel(table_name = users_roles)]
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
