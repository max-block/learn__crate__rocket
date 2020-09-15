#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;

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
    rocket::ignite()
        .mount("/", routes![index_h, welcome_h, hello_h])
        .launch();
}
