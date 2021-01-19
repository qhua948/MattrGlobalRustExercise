#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Credential {
}

#[get("/credentials")]
fn get_credentials() -> Json<Vec<Credential>> {
    Json(Vec::new())
}

fn main() {
    rocket::ignite()
        .mount("/",
            routes![get_credentials]).launch();
}
