use rocket::{
    response::status::{Created, NoContent},
    serde::json::Json,
};

use crate::{
    models::members::*,
    errors::*,
    database::{PgConnection}
};

#[get("/")]
pub async fn list(connection: PgConnection) -> Result<Json<Vec<Member>>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(Json)
}

#[get("/<id>")]
pub async fn retrieve(
    connection: PgConnection,
    id: i32,
) -> Result<Json<Member>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(Json)
}

#[post("/", data = "<member>")]
pub async fn create(
    connection: PgConnection,
    member: Json<NewMember>,
) -> Result<Created<Json<Member>>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(|a| Created::new("/").body(Json(a)))
}

#[patch("/<id>", data = "<member>")]
pub async fn update(
    connection: PgConnection,
    id: i32,
    member: Json<UpdatedMember>,
) -> Result<Json<Member>, Error> {
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
