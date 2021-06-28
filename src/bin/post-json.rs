#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;

use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Debug)]
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
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
