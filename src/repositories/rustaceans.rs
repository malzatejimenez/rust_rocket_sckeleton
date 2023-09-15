use crate::{
    models::rustaceans::*,
    schema::rustaceans::{self as rustacean_schema, table as rustaceans_table},
};
use diesel::{prelude::*, PgConnection, QueryResult};

pub struct RustaceanRepository;

impl RustaceanRepository {
    // Encuentra un rustacean por su ID
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans_table.find(id).get_result(c)
    }

    // Encuentra múltiples rustaceans limitados por una cantidad específica
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans_table.limit(limit).load(c)
    }

    // Crea un nuevo rustacean en la base de datos
    pub fn create(c: &mut PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans_table)
            .values(new_rustacean)
            .get_result(c)
    }

    // Actualiza los datos de un rustacean existente por su ID
    pub fn update(c: &mut PgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans_table.find(id))
            .set((
                rustacean_schema::name.eq(rustacean.name),
                rustacean_schema::email.eq(rustacean.email),
            ))
            .get_result(c)
    }

    // Elimina un rustacean por su ID
    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans_table.find(id)).execute(c)
    }
}
