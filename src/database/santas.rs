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

pub fn recipient_id(
    group_id: i32,
    santa_id: i32,
    c: &diesel::PgConnection 
) -> Result<i32, Error> {
    let ret = santas::table
        .filter(santas::group_id.eq(group_id))
        .filter(santas::santa_id.eq(santa_id))
        .select(santas::recipient_id)
        .get_result(c);
    match ret {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

pub fn list(c: &diesel::PgConnection) -> Result<Vec<Santa>, Error> {
    match santas::table.load(c) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string()))
    }
}

pub fn retrieve(
    id: i32,
    c: &diesel::PgConnection
) -> Result<Santa, Error> {
    check_santa_id(id, c)?;
    match santas::table.filter(santas::id.eq(id)).first(c) {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string()))
    }
}

pub fn create(
    santa: &NewSanta,
    c: &diesel::PgConnection
) -> Result<Santa, Error> {
    let ret = diesel::insert_into(santas::table)
        .values(santa)
        .get_result(c);
    match ret {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string()))
    }
}

pub fn create_values(
    santas: &Vec<NewSanta>,
    c: &diesel::PgConnection
) -> Result<Vec<Santa>, Error> {
    let ret = diesel::insert_into(santas::table)
        .values(santas)
        .load(c);
    match ret {
        Ok(o) => Ok(o),
        Err(e) => Err(Error::Internal(e.to_string()))
    }
}

pub fn update(
    id: i32,
    santa: &UpdatedSanta,
    c: &diesel::PgConnection
) -> Result<Santa, Error> {
    check_santa_id(id, c)?;
    let ret = diesel::update(santas::table.find(id))
        .set(santa)
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
        santas::table.filter(santas::id.eq(id)))
        .execute(c);
    match ret {
        Ok(o) => {
            match o {
                1 => Ok(()),
                0 => Err(Error::NotFound(format!("Santa id:{id} is not found"))),
                _ => Err(Error::BadRequest("???".to_string())),
            }
        }
        Err(e) => Err(Error::Internal(e.to_string())),
    }
}

fn generated_santas(group_id: i32, ids: &Vec<i32>) -> Result<Vec<NewSanta>, Error> {
    let itrs = ids.len();
    if itrs <= 1 {
        return Err(Error::Internal(format!(
            "There are not enough members in group id:{group_id}"
        ).to_string()));
    }
    let mut ids1 = ids.clone();
    let mut ids2 = ids.clone();
    let mut rng = rand::thread_rng();
    let mut ret = Vec::new();
    for _ in 2..itrs {
        let len = ids1.len();
        let r = rng.gen::<usize>() % len; 
        let mut c = rng.gen::<usize>() % len;
        c = if ids1[r] != ids2[c] { c } else { (c + 1) % len };
        ret.push(NewSanta { 
            group_id: group_id,
            santa_id: ids1[r],
            recipient_id: ids2[c],
        });
        ids1.swap_remove(r);
        ids2.swap_remove(c);
    }
    let c =
    if ids1[0] == ids2[0] || ids1[1] == ids2[1] { 1 } else { 0 };
    ret.push(NewSanta { 
        group_id: group_id,
        santa_id: ids1[0],
        recipient_id: ids2[c],
    });
    ret.push(NewSanta {
        group_id: group_id,
        santa_id: ids1[1],
        recipient_id: ids2[(c + 1) % 2],
    });
    Ok(ret)
}

pub fn allocate(
    user_id: i32,
    group_id: i32,
    c: &diesel::PgConnection
) -> Result<(), Error> {
    database::users::check_user_id(user_id, c)?;
    database::groups::check_group_id(group_id, c)?;
    database::members::check_user_and_group_id(user_id, group_id, c)?;
    database::members::check_permission(user_id, group_id, c)?;
    database::groups::check_close(group_id, c)?;

    let ids = database::members::list_group_users(group_id, c)?;
    let santas = generated_santas(group_id, &ids)?;
    create_values(&santas, c)?;

    database::groups::update(
        group_id,
        &UpdatedGroup { name: None, is_close: Some(true) },
        c
    )?;

    Ok(())
}

pub fn recipient(
    santa_id: i32,
    group_id: i32,
    c: &diesel::PgConnection
) -> Result<NamedMember, Error> {
    database::users::check_user_id(santa_id, c)?;
    database::groups::check_group_id(group_id, c)?;
    database::members::check_user_and_group_id(santa_id, group_id, c)?;
    
    let rec_id = database::santas::recipient_id(group_id, santa_id, c)?;

    database::members::named_member(rec_id, group_id, c)
}
