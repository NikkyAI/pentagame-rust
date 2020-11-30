// imports
use crate::frontend::errors::UserError;
use crate::frontend::routes::DbPool;
use crate::ws::errors::WebsocketError;
use actix_web::web::Data;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;

// conditional imports
use crate::api::errors::APIError;

// types
type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

/*
 removes trailing null characters from String
 Required for db operations, because Postgresql TEXT field doesn't support tailing NULL characters
*/
pub fn zero_trim(s: &String) -> String {
    s.trim_matches(char::from(0)).to_owned()
}

pub fn acquire_connection_user(pool: &Data<DbPool>) -> Result<DbConnection, UserError> {
    match pool.get() {
        Ok(connection) => Ok(connection),
        Err(_) => Err(UserError::InternalError {
            code: 3,
            message: "Failed to acquire connection to database".to_owned(),
        }),
    }
}

pub fn acquire_connection_api(pool: &Data<DbPool>) -> Result<DbConnection, APIError> {
    match pool.get() {
        Ok(connection) => Ok(connection),
        Err(_) => Err(APIError::DataBasePoolError {
            message: "Failed to acquire connection to database".to_owned(),
        }),
    }
}

pub fn acquire_connection_ws(pool: &DbPool) -> Result<DbConnection, APIError> {
    match pool.get() {
        Ok(connection) => Ok(connection),
        Err(_) => Err(APIError::DataBasePoolError {
            message: "Failed to acquire a connection from database".to_owned(),
        }),
    }
}
