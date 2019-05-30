#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod controller;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            controller::routes::get_all,
            controller::routes::get,
            controller::routes::add,
            controller::routes::delete,
            controller::routes::delete_all,
            controller::routes::update
        ])
        .launch();
}