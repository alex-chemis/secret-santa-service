use rocket::{
    response::status::{Created, NoContent},
    serde::json::Json,
};

use crate::{
    models::{santas::*},
    errors::*,
    database::{PgConnection, santas},
};

#[get("/")]
pub async fn list(connection: PgConnection) -> Result<Json<Vec<Santa>>, Error> {
    connection
        .run(|c| santas::list(c))
        .await
        .map(Json)
}

#[get("/<id>")]
pub async fn retrieve(
    connection: PgConnection,
    id: i32,
) -> Result<Json<Santa>, Error> {
    connection
        .run(move |c| santas::retrieve(id, c))
        .await
        .map(Json)
}

#[post("/", data = "<santa>")]
pub async fn create(
    connection: PgConnection,
    santa: Json<NewSanta>,
) -> Result<Created<Json<Santa>>, Error> {
    connection
        .run(move |c| santas::create(&santa, c))
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
        .run(move |c| santas::update(id, &santa, c))
        .await
        .map(Json)
}

#[delete("/<id>")]
pub async fn destroy(
    connection: PgConnection,
    id: i32
) -> Result<NoContent, Error> {
    connection
        .run(move |c| santas::destroy(id, c))
        .await
        .map(|_| NoContent)
}
