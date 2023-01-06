use rocket::{
    response::status::{Created, NoContent},
    serde::json::Json,
};

use crate::{
    models::{groups::*, members::*},
    errors::*,
    database::{PgConnection, groups, members}
};

#[get("/")]
pub async fn list(connection: PgConnection) -> Result<Json<Vec<Group>>, Error> {
    connection
        .run(move |c| groups::list(c))
        .await
        .map(Json)
}

#[get("/<id>")]
pub async fn retrieve(
    connection: PgConnection,
    id: i32,
) -> Result<Json<Group>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(Json)
}

#[post("/", data = "<group>")]
pub async fn create(
    connection: PgConnection,
    group: Json<NewGroup>,
) -> Result<Created<Json<Group>>, Error> {
    connection
        .run(move |c| groups::create(&group, c))
        .await
        .map(|a| Created::new("/").body(Json(a)))
}

#[patch("/<id>", data = "<group>")]
pub async fn update(
    connection: PgConnection,
    id: i32,
    group: Json<UpdatedGroup>,
) -> Result<Json<Group>, Error> {
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
        .run(move |c| groups::destroy(id, c))
        .await
        .map(|_| NoContent)
}

#[get("/<id>/members")]
pub async fn list_group_members(
    connection: PgConnection,
    id: i32
) -> Result<Json<Vec<NamedMember>>, Error> {
    connection
        .run(move |c| members::list_group_members(id, c))
        .await
        .map(Json)
}

#[get("/<id>/admins")]
pub async fn list_group_admins(
    connection: PgConnection,
    id: i32
) -> Result<Json<Vec<NamedMember>>, Error> {
    connection
        .run(move |c| members::list_group_admins(id, c))
        .await
        .map(Json)
}
