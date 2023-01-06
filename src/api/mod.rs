use rocket::serde::{Deserialize, Serialize};

pub mod groups;
pub mod members;
pub mod santas;
pub mod users;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ApiError {
    pub details: String,
}