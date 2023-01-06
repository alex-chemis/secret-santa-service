use rocket::{
    response::status::{Created, NoContent},
    serde::json::Json,
};

use crate::{
    models::{users::*, groups::*, members::*},
    errors::*,
    database::{PgConnection, users},
};

#[get("/")]
pub async fn list(connection: PgConnection) -> Result<Json<Vec<User>>, Error> {
    connection
        .run(|c| users::list(c))
        .await
        .map(Json)
}

#[get("/<id>")]
pub async fn retrieve(
    connection: PgConnection,
    id: i32,
) -> Result<Json<User>, Error> {
    connection
        .run(move |c| users::retrieve(id, c))
        .await
        .map(Json)
}

#[post("/", data = "<user>")]
pub async fn create(
    connection: PgConnection,
    user: Json<NewUser>,
) -> Result<Created<Json<User>>, Error> {
    connection
        .run(move |c| users::create(&user, c))
        .await
        .map(|a| Created::new("/").body(Json(a)))
}

#[patch("/<id>", data = "<user>")]
pub async fn update(
    connection: PgConnection,
    id: i32,
    user: Json<UpdatedUser>,
) -> Result<Json<User>, Error> {
    connection
        .run(move |c| users::update(id, &user, c))
        .await
        .map(Json)
}

#[delete("/<id>")]
pub async fn destroy(
    connection: PgConnection,
    id: i32
) -> Result<NoContent, Error> {
    connection
        .run(move |c| users::destroy(id, c))
        .await
        .map(|_| NoContent)
}

#[post("/<id>/groups", data = "<group>")]
pub async fn create_group(
    connection: PgConnection,
    id: i32,
    group: Json<NewGroup>,
) -> Result<Created<Json<Group>>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(|a| Created::new("/").body(Json(a)))
}

#[delete("/<id>/groups/<group_id>")]
pub async fn destroy_group(
    connection: PgConnection,
    id: i32,
    group_id: i32,
) -> Result<NoContent, Error> {
    connection
        .run(move |c| { Err::<(), Error>(Error::NotFound("".to_string())) })
        .await
        .map(|_| NoContent)
}

#[put("/<id>/groups/<group_id>/join")]
pub async fn join_group(
    connection: PgConnection,
    id: i32,
    group_id: i32
) -> Result<Created<Json<Member>>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(|a| Created::new("/").body(Json(a)))
}

#[put("/<id>/groups/<group_id>/leave")]
pub async fn leave_group(
    connection: PgConnection,
    id: i32,
    group_id: i32
) -> Result<NoContent, Error> {
    connection
        .run(move |c| { Err::<(), Error>(Error::NotFound("".to_string())) })
        .await
        .map(|_| NoContent)
}

#[put("/<id>/groups/<group_id>/members/<member_id>/admin")]
pub async fn admin_member(
    connection: PgConnection,
    id: i32,
    group_id: i32,
    member_id: i32
) -> Result<NoContent, Error> {
    connection
        .run(move |c| { Err::<(), Error>(Error::NotFound("".to_string())) })
        .await
        .map(|_| NoContent)
}

#[put("/<id>/groups/<group_id>/unadmin")]
pub async fn unadmin_self(
    connection: PgConnection,
    id: i32,
    group_id: i32,
) -> Result<Json<Member>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(Json)
}

#[put("/<id>/groups/<group_id>/allocate")]
pub async fn allocate(
    connection: PgConnection,
    id: i32,
    group_id: i32,
) -> Result<NoContent, Error> {
    connection
        .run(move |c| { Err::<(), Error>(Error::NotFound("".to_string())) })
        .await
        .map(|_| NoContent)
}

#[get("/<id>/groups/<group_id>/recipient")]
pub async fn recipient(
    connection: PgConnection,
    id: i32,
    group_id: i32,
) -> Result<Json<NamedMember>, Error> {
    connection
        .run(move |c| { Err(Error::NotFound("".to_string())) })
        .await
        .map(Json)
}
