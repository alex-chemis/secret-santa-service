#[macro_use]
extern crate rocket;

use secret_santa_service_example::database::PgConnection;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgConnection::fairing())
}