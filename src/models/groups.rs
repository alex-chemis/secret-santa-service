use rocket::serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Insertable, Queryable};

use crate::schema::groups;

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub is_close: bool,
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "groups"]
pub struct NewGroup {
    pub name: String,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "groups"]
pub struct UpdatedGroup {
    pub name: Option<String>,
    pub is_close: Option<bool>,
}