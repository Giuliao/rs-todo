use actix_web::dev::Payload;
use actix_web::error::ErrorServiceUnavailable;
use actix_web::{Error, FromRequest, HttpRequest};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenvy::dotenv;
use futures::future::{err, ok, Ready};
use lazy_static::lazy_static;
use std::env;

use crate::config::Config;

type PgPool = Pool<ConnectionManager<PgConnection>>;
pub struct DbConnection {
    pub db_connection: PgPool,
}

lazy_static! {
    pub static ref DBCONNECTION: DbConnection = {
        dotenv().ok();
        let connection_string = match Config::new().map.get("DATABASE_URL") {
            Some(val) => match val.as_str() {
                Some(v) => v.to_string(),
                _ => env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string()),
            },
            _ => env::var("DATABASE_URL").unwrap_or_else(|_| "".to_string()),
        };

        DbConnection {
            db_connection: PgPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(connection_string))
                .expect("failed to create db connection_pool"),
        }
    };
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    DBCONNECTION.db_connection.get().unwrap()
}

pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<DB, Error>>;
    fn from_request(_: &HttpRequest, _: &mut Payload) -> Self::Future {
        match DBCONNECTION.db_connection.get() {
            Ok(connection) => ok(DB { connection }),
            Err(_) => err(ErrorServiceUnavailable(
                "could not make connection to database",
            )),
        }
    }
}
