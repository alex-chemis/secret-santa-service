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

pub fn list(c: &diesel::PgConnection) -> Result<Vec<Group>, Error> {
    match groups::table.load(c) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string()))
    }
}

pub fn retrieve(
    id: i32,
    c: &diesel::PgConnection
) -> Result<Group, Error> {
    check_group_id(id, c)?;
    match groups::table.filter(groups::id.eq(id)).first(c) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string()))
    }
}

pub fn create(
    group: &NewGroup,
    c: &diesel::PgConnection
) -> Result<Group, Error> {
    let ret = diesel::insert_into(groups::table)
        .values(group)
        .get_result(c);
    match ret {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

pub fn update(
    id: i32,
    group: &UpdatedGroup,
    c: &diesel::PgConnection
) -> Result<Group, Error> {
    check_group_id(id, c)?;
    let ret = diesel::update(groups::table.find(id))
        .set(group)
        .get_result(c);
    match ret {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

pub fn destroy(
    id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = diesel::delete(
        groups::table.filter(groups::id.eq(id)))
        .execute(c);
    match ret {
        Ok(o) => {
            match o {
                1 => Ok(()),
                0 => Err(Error::NotFound(format!("Group id:{id} is not found"))),
                _ => Err(Error::BadRequest("???".to_string())),
            }
        }
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}
