use diesel::{prelude::*};
use rand::Rng;

use crate::{
    errors::*,
    database,
    models::{santas::*, groups::*, members::*},
    schema::{santas},
};

pub fn check_santa_id(
    id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = santas::table
        .filter(santas::id.eq(id))
        .execute(c);
    match ret {
        Ok(o) => match o {
                1 => Ok(()),
                0 => Err(Error::NotFound(format!("Member id:{id} is not found").to_string())),
                _ => Err(Error::BadRequest("???".to_string())),
            },
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}