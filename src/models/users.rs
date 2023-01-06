use rocket::serde::{Deserialize, Serialize};
use diesel::{AsChangeset, Insertable, Queryable};

use crate::{schema::users};

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct UpdatedUser {
    pub name: Option<String>,
}
