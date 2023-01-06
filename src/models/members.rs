use rocket::serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Insertable, Queryable};

use crate::schema::members;

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Member {
    pub id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub is_admin: bool,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct PartialMember {
    pub id: i32,
    pub group_id: i32,
    pub is_admin: bool,
}

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NamedMember {
    pub id: i32,
    pub name: String,
    pub group_id: i32,
    pub is_admin: bool,
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "members"]
pub struct NewMember {
    pub user_id: i32,
    pub group_id: i32,
    pub is_admin: bool,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "members"]
pub struct UpdatedMember {
    pub is_admin: Option<bool>,
}
