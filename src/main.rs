#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!, from '/'"
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!, from '/hello/world'"
}

fn main() {
    // rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/hello", routes![world]).launch();
}
