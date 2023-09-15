use crate::{
    models::crates::*,
    schema::crates::{self as crate_schema, table as crates_table},
};
use diesel::{prelude::*, PgConnection, QueryResult};

pub struct CrateRepository;

impl CrateRepository {
    // Encuentra un crate por su ID
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates_table.find(id).get_result(c)
    }

    // Encuentra múltiples crates limitados por una cantidad específica
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates_table.limit(limit).load(c)
    }

    // Crea un nuevo crate en la base de datos
    pub fn create(c: &mut PgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates_table)
            .values(new_crate)
            .get_result(c)
    }

    // Actualiza los datos de un crate existente por su ID
    pub fn update(c: &mut PgConnection, id: i32, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates_table.find(id))
            .set((
                crate_schema::rustacean_id.eq(a_crate.rustacean_id),
                crate_schema::code.eq(a_crate.code),
                crate_schema::name.eq(a_crate.name),
                crate_schema::version.eq(a_crate.version),
                crate_schema::description.eq(a_crate.description),
            ))
            .get_result(c)
    }

    // Elimina un crate por su ID
    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates_table.find(id)).execute(c)
    }
}
