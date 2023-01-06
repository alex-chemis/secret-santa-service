pub mod groups;
pub mod members;
pub mod santas;
pub mod users;

use rocket_sync_db_pools::database;

#[database("db")]
pub struct PgConnection(diesel::PgConnection);
