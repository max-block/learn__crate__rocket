#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "it works"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

