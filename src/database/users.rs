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

pub fn list(c: &diesel::PgConnection) -> Result<Vec<User>, Error> {
    match users::table.load(c) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string()))
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

pub fn destroy(
    id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    let ret = diesel::delete(
        users::table.filter(users::id.eq(id)))
        .execute(c);
    match ret {
        Ok(o) => {
            match o {
                1 => Ok(()),
                0 => Err(Error::NotFound(format!("Users id:{id} is not found"))),
                _ => Err(Error::BadRequest("???".to_string())),
            }
        }
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

pub fn create_group(
    user_id: i32,
    group: &NewGroup,
    c: &diesel::PgConnection
) -> Result<Group, Error> {
    check_user_id(user_id, c)?;

    let ret = database::groups::create(&group, c)?;
    database::members::create_admin(user_id, ret.id, c)?;
    Ok(ret)
}

pub fn destroy_group(
    user_id: i32,
    group_id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    check_user_id(user_id, c)?;
    database::groups::check_group_id(group_id, c)?;
    database::members::check_user_and_group_id(user_id, group_id, c)?;
    database::members::check_permission(user_id, group_id, c)?;

    database::groups::destroy(group_id, c)
}

pub fn join_group(
    user_id: i32,
    group_id: i32,
    c: &diesel::PgConnection
) -> Result<Member, Error> {
    check_user_id(user_id, c)?;
    database::groups::check_group_id(group_id, c)?;
    database::groups::check_close(group_id, c)?;

    database::members::create(&NewMember {
        user_id: user_id,
        group_id: group_id,
        is_admin:
        false 
    }, c)
}

pub fn leave_group(
    user_id: i32,
    group_id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    check_user_id(user_id, c)?;
    database::groups::check_group_id(group_id, c)?;
    database::members::check_user_and_group_id(user_id, group_id, c)?;
    database::members::check_leave(user_id, group_id, c)?;
    database::groups::check_close(group_id, c)?;

    database::members::destroy_user_group_id(user_id, group_id, c)
}

pub fn admin_member(
    user_id: i32,
    group_id: i32,
    member_id: i32,
    c: &diesel::PgConnection
) -> Result<Member, Error> {
    check_user_id(user_id, c)?;
    database::groups::check_group_id(group_id, c)?;
    database::members::check_user_and_group_id(user_id, group_id, c)?;
    database::members::check_permission(user_id, group_id, c)?;
    database::members::check_member_id(member_id, c)?;
    database::groups::check_close(group_id, c)?;

    database::members::update(member_id, &UpdatedMember {
        is_admin: Some(false) 
    }, c)
}

pub fn unadmin_self(
    user_id: i32,
    group_id: i32,
    c: &diesel::PgConnection
) -> Result<Member, Error> {
    check_user_id(user_id, c)?;
    database::groups::check_group_id(group_id, c)?;
    database::members::check_user_and_group_id(user_id, group_id, c)?;
    database::members::check_permission(user_id, group_id, c)?;
    database::members::check_admins(group_id, c)?;
    database::groups::check_close(group_id, c)?;

    database::members::update_user_group_id(user_id, group_id, &UpdatedMember {
        is_admin: Some(false) 
    }, c)
}
