use rocket::{
    response::status::{Created, NoContent},
    serde::json::Json,
};

use crate::{
    models::{santas::*},
    errors::*,
    database::{PgConnection},
};

#[get("/")]
pub async fn list(connection: PgConnection) -> Result<Json<Vec<Santa>>, Error> {
    connection
        .run(|c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(Json)
}

#[get("/<id>")]
pub async fn retrieve(
    connection: PgConnection,
    id: i32,
) -> Result<Json<Santa>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(Json)
}

#[post("/", data = "<santa>")]
pub async fn create(
    connection: PgConnection,
    santa: Json<NewSanta>,
) -> Result<Created<Json<Santa>>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(|a| Created::new("/").body(Json(a)))
}

#[patch("/<id>", data = "<santa>")]
pub async fn update(
    connection: PgConnection,
    id: i32,
    santa: Json<UpdatedSanta>,
) -> Result<Json<Santa>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(Json)
}

#[delete("/<id>")]
pub async fn destroy(
    connection: PgConnection,
    id: i32
) -> Result<NoContent, Error> {
    connection
        .run(move |c| { Err::<(), Error>(Error::NotFound("".to_string())) })
        .await
        .map(|_| NoContent)
}
