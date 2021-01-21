use std::io::Cursor;

use rocket::{Response, State};
use rocket::http::{Status, ContentType};
use rocket_contrib::json::Json;

use crate::Conf;
use crate::dao::ConnectionRestMapping;
use crate::datastructures::{CryptographicKeys, ErrorMessage};

use super::internal::*;

#[get("/?<limit>&<offset>")]
pub fn get_cryptographic_keys(state: State<Conf>, limit: Option<u32>, offset: Option<u32>) -> Response {
    return generic_get(state, limit, offset, Box::new( CryptographicKeys::get_all));
}

#[get("/<id>")]
pub fn get_cryptographic_key_by_id(state: State<Conf>, id: u32) -> Response {
    return generic_get_by_id(state, id, Box::new(CryptographicKeys::get_by_id));
}

#[post("/", data = "<ck>")]
pub fn create_cryptographic_key(state: State<Conf>, ck: Json<CryptographicKeys>) -> Response {
    if ck.public_key.is_none() {
        let mut response = Response::build();
        response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Invalid public_key field" }).unwrap()))
            .status(Status::InternalServerError);
        response.header(ContentType::JSON).finalize()
    } else {
        generic_create(state, ck, Box::new(CryptographicKeys::create))
    }
}

#[put("/", data = "<ck>")]
pub fn update_cryptographic_key(state: State<Conf>, ck: Json<CryptographicKeys>) -> Response {
    if ck.public_key.is_some() && ck.id.is_some() {
        generic_update(state, ck, Box::new(CryptographicKeys::update))
    } else {
        let mut response = Response::build();
        response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Invalid request body" }).unwrap()))
            .status(Status::UnprocessableEntity);
        response.header(ContentType::JSON).finalize()
    }
}

#[delete("/<id>")]
pub fn delete_cryptographic_key(state: State<Conf>, id: u32) -> Response {
    return generic_delete::<CryptographicKeys>(state, id, Box::new(CryptographicKeys::delete_by_id));
}
