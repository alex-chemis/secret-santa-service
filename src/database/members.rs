use diesel::{prelude::*};

use crate::{
    errors::*,
    models::{members::*},
    schema::{members, users},
};

pub fn check_member_id(
    id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = members::table
        .filter(members::id.eq(id))
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

pub fn check_user_and_group_id(
    user_id: i32,
    group_id: i32, 
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = members::table
        .filter(members::user_id.eq(user_id))
        .filter(members::group_id.eq(group_id))
        .execute(c);
    match ret {
        Ok(o) => match o {
                1 => Ok(()),
                0 => Err(Error::NotFound(format!(
                    "User id:{user_id} is not found in group id:{group_id}"
                ).to_string())),
                _ => Err(Error::BadRequest("???".to_string())),
            },
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

pub fn check_permission(
    user_id: i32,
    group_id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = members::table
        .filter(members::user_id.eq(user_id))
        .filter(members::group_id.eq(group_id))
        .filter(members::is_admin.eq(true))
        .execute(c);
    match ret {
        Ok(o) => match o {
                1 => Ok(()),
                0 => Err(Error::Forbidden(format!(
                    "User id:{user_id} does not have administrator rights in the group id:{group_id}"
                ).to_string())),
                _ => Err(Error::BadRequest("???".to_string())),
            },
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

pub fn check_admins(
    group_id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = members::table
        .filter(members::group_id.eq(group_id))
        .filter(members::is_admin.eq(true))
        .execute(c);
    match ret {
        Ok(o) => {
            if o > 1 {
                Ok(())
            } else {
                Err(Error::Forbidden(format!(
                    "There are not enough admins in group id:{group_id}"
                ).to_string()))
            }
        }
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

pub fn check_leave(
    user_id: i32,
    group_id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    match check_permission(user_id, group_id, c) {
        Ok(_) => check_admins(group_id, c),
        Err(e) => match e {
            Error::Forbidden(_) => Ok(()),
            _ => Err(e)
        }
    }
}