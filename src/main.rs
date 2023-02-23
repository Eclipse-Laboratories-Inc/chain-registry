#[macro_use] extern crate rocket;
use eclipse_chain_registry::pool::Db;
use sea_orm_rocket::Database;




#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::init())
    .mount("/", routes![])
}