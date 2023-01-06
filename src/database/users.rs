use diesel::prelude::*;

use crate::{
    errors::*,
    database,
    models::{users::*, groups::*, members::*},
    schema::{users},
};

pub fn check_user_id(
    id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = users::table
        .filter(users::id.eq(id))
        .execute(c);
    match ret {
        Ok(o) => match o {
                1 => Ok(()),
                0 => Err(Error::NotFound(format!("User id:{id} is not found").to_string())),
                _ => Err(Error::BadRequest("???".to_string())),
            },
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}