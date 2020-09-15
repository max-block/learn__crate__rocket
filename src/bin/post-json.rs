#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct CreateData {
    name: String,
    value: i32,
}

#[post("/", format = "json", data = "<new_data>")]
fn index(new_data: Json<CreateData>) -> String {
    println!("{:#?}", new_data);
    "ok".to_string()
}

// curl -X POST -H "Content-Type: application/json" -d '{"name": "v1", "value": 333}' http://localhost:8000
fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
