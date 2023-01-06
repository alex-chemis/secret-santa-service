use diesel::{prelude::*};

use crate::{
    errors::*,
    models::{groups::*},
    schema::{groups},
};

pub fn check_group_id(
    id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = groups::table
        .filter(groups::id.eq(id))
        .execute(c);
    match ret {
        Ok(o) => match o {
                1 => Ok(()),
                0 => Err(Error::NotFound(format!("Group id:{id} is not found").to_string())),
                _ => Err(Error::BadRequest("???".to_string())),
            },
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}
