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

pub fn retrieve(
    id: i32,
    c: &diesel::PgConnection
) -> Result<User, Error> {
    check_user_id(id, c)?;
    match users::table.filter(users::id.eq(id)).first(c) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string()))
    }
}

pub fn create(
    user: &NewUser,
    c: &diesel::PgConnection
) -> Result<User, Error> {
    let ret = diesel::insert_into(users::table)
        .values(user)
        .get_result(c);
    match ret {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string()))
    }
}

pub fn update(
    id: i32,
    user: &UpdatedUser,
    c: &diesel::PgConnection
) -> Result<User, Error> {
    check_user_id(id, c)?;
    let ret = diesel::update(users::table.find(id))
        .set(user)
        .get_result(c);
    match ret {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}