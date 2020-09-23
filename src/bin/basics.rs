#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{config::Environment, response::Redirect, Config};

#[get("/")]
fn index_h() -> Redirect {
    Redirect::to(uri!(welcome_h))
}

#[get("/welcome")]
fn welcome_h() -> &'static str {
    "it works"
}

#[get("/hello/<name>")]
fn hello_h(name: String) -> String {
    format!("Hello, {}", name)
}

fn main() {
    let config = Config::build(Environment::Development)
        .address("localhost")
        .port(3000)
        .finalize()
        .unwrap();
    rocket::custom(config)
        .mount("/", routes![index_h, welcome_h, hello_h])
        .launch();
}
