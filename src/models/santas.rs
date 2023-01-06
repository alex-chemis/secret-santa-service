use rocket::serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Insertable, Queryable};

use crate::schema::santas;

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Santa {
    pub id: i32,
    pub group_id: i32,
    pub santa_id: i32,
    pub recipient_id: i32,
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "santas"]
pub struct NewSanta {
    pub group_id: i32,
    pub santa_id: i32,
    pub recipient_id: i32,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "santas"]
pub struct UpdatedSanta {
    pub group_id: Option<i32>,
    pub santa_id: Option<i32>,
    pub recipient_id: Option<i32>,
}
