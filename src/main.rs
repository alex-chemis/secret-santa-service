#[macro_use]
extern crate rocket;

use secret_santa_service_example::{api::*, database::PgConnection};
use rand::Rng;

#[launch]
fn rocket() -> _ {
    let mut rng = rand::thread_rng();
    let service_key = rng.gen::<i64>() % 9_000_000 + 1_000_000;
    
    rocket::build()
        .attach(PgConnection::fairing())
        .mount(
            format!("/{service_key}/users"),
            routes![
                users::list,
                users::retrieve,
                users::create, 
                users::update, 
                users::destroy,
            ]
        )
        .mount(
            format!("/{service_key}/groups"),
            routes![
                groups::list,
                groups::retrieve,
                groups::create, 
                groups::update, 
                groups::destroy,
            ]
        )
        .mount(
            format!("/{service_key}/members"),
            routes![
                members::list,
                members::retrieve,
                members::create, 
                members::update, 
                members::destroy,
            ]
        )
        .mount(format!("/{service_key}/santas"),
            routes![
                santas::list,
                santas::retrieve,
                santas::create,
                santas::update,
                santas::destroy,
            ]
        )
        .mount(
            "/users",
            routes![
                users::retrieve,
                users::create, 
                users::update, 
                users::destroy,
                users::create_group,
                users::destroy_group,
                users::join_group,
                users::leave_group,
                users::admin_member,
                users::unadmin_self,
                users::allocate,
                users::recipient,
            ]
        )
        .mount(
            "/groups",
            routes![
                groups::list,
                groups::retrieve,
                groups::list_group_members,
                groups::list_group_admins
            ]
        )
}
