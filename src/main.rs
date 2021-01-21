#![feature(proc_macro_hygiene, decl_macro)]
#![feature(box_patterns)]

#[macro_use]
extern crate rocket;

mod routes;
mod dao;
mod datastructures;
mod test;

use rusqlite::Connection;
use rocket::Rocket;

pub struct Conf {
    db_file_path: &'static str,
}

impl Conf {
    fn get_new_db_connection(&self) -> Connection {
        Connection::open(self.db_file_path).unwrap()
    }
}

fn get_ignited_rocket() -> Rocket {
    rocket::ignite()
        .manage(Conf { db_file_path: "db" })
        .mount("/credentials", routes![
            routes::credentials::get_credentials,
            routes::credentials::get_credential_by_id,
            routes::credentials::create_credential,
            routes::credentials::delete_credential,
            routes::credentials::update_credential,
            ])
        .mount("/schemas", routes![
            routes::schemas::get_schemas,
            routes::schemas::get_schema_by_id,
            routes::schemas::create_schema,
            routes::schemas::delete_schema,
            routes::schemas::update_schema,
        ])
        .mount("/cryptographic_keys", routes![
            routes::cryptographic_keys::get_cryptographic_keys,
            routes::cryptographic_keys::get_cryptographic_key_by_id,
            routes::cryptographic_keys::create_cryptographic_key,
            routes::cryptographic_keys::delete_cryptographic_key,
            routes::cryptographic_keys::update_cryptographic_key,
        ])
}

fn main() {
        get_ignited_rocket().launch();
}
