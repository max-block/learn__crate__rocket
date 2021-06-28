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

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("address", "127.0.0.1"))
        .merge(("port", 3000));

    rocket::custom(figment).mount("/", routes![index_h, welcome_h, hello_h])
}
