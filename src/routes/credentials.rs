use std::io::Cursor;

use rocket::{Response, State};
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::Conf;
use crate::dao::ConnectionRestMapping;
use crate::datastructures::{conforms, Credential, CryptographicKeys, ErrorMessage, Schema};

use super::internal::*;

// Dummy function in place of verification
fn verify(_cred: &Credential, _key: &CryptographicKeys) -> bool {
    // Compare (fingerprint) == f(data + public key)
    true
}

#[get("/?<limit>&<offset>")]
pub fn get_credentials(state: State<Conf>, limit: Option<u32>, offset: Option<u32>) -> Response {
    generic_get(state, limit, offset, Box::new(Credential::get_all))
}

#[get("/<id>")]
pub fn get_credential_by_id(state: State<Conf>, id: u32) -> Response {
    generic_get_by_id(state, id, Box::new(Credential::get_by_id))
}

/// Check the object to ensure conforms to schema and has keys
/// returns Some(Response) when check fails which shall be returned
pub fn check_request_credential<'a>(state: &'a State<Conf>, cd: &Json<Credential>) -> Option<Response<'static>> {
    let conn = state.get_new_db_connection();
    let mut response = json_response();
    // Check for schema
    if cd.schema_id.is_none() {
        response.sized_body(
            Cursor::new(serde_json::to_string(&ErrorMessage { error: "Invalid schema_id field" }).unwrap()))
            .status(Status::BadRequest);
        return Some(response.finalize());
    }
    let schema_id = cd.schema_id.unwrap();
    // Check is data conforms to data
    match Schema::get_by_id(&conn, schema_id) {
        Ok(Some(ref schema)) => {
            if schema.schema.is_none() || !conforms(&cd.0, &schema.schema.clone().unwrap()) {
                return Some(response.sized_body(
                    Cursor::new(serde_json::to_string(&ErrorMessage { error: "Invalid or Nonconforming schema" }).unwrap()))
                    .status(Status::BadRequest).finalize());
            }
        }
        _ => {
            return Some(response.sized_body(
                Cursor::new(serde_json::to_string(&ErrorMessage { error: "No schema found" }).unwrap()))
                .status(Status::BadRequest).finalize());
        }
    }
    None
}

#[post("/", data = "<cd>")]
pub fn create_credential(state: State<Conf>, cd: Json<Credential>) -> Response {
    if let Some(res) = check_request_credential(&state, &cd) {
        return res;
    }
    generic_create(state, cd, Box::new(Credential::create))
}

#[put("/", data = "<cd>")]
pub fn update_credential(state: State<Conf>, cd: Json<Credential>) -> Response {
    let conn = state.get_new_db_connection();
    let mut response = json_response();
    // Check for schema
    if cd.schema_id.is_none() {
        response.sized_body(
            Cursor::new(serde_json::to_string(&ErrorMessage { error: "Invalid schema_id field" }).unwrap()))
            .status(Status::BadRequest);
        return response.finalize();
    }
    let schema_id = cd.schema_id.unwrap();
    // Check is data conforms to data
    match Schema::get_by_id(&conn, schema_id) {
        Ok(Some(ref schema)) => {
            if schema.schema.is_none() || !conforms(&cd.0, &schema.schema.clone().unwrap()) {
                return response.sized_body(
                    Cursor::new(serde_json::to_string(&ErrorMessage { error: "Invalid or Nonconforming schema" }).unwrap()))
                    .status(Status::BadRequest).finalize();
            }
        }
        _ => {
            return response.sized_body(
                Cursor::new(serde_json::to_string(&ErrorMessage { error: "No schema found" }).unwrap()))
                .status(Status::BadRequest).finalize();
        }
    }

    if cd.public_key_id.is_none() {
        return response.sized_body(
            Cursor::new(serde_json::to_string(&ErrorMessage { error: "Invalid public_id field" }).unwrap()))
            .status(Status::BadRequest).finalize();
    }
    let public_key_id = cd.public_key_id.unwrap();
    match CryptographicKeys::get_by_id(&conn, public_key_id) {
        Ok(Some(ref k)) => {
            if k.public_key.is_none() || verify(&cd, k) {
                response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Invalid or Nonconforming schema" }).unwrap()))
                    .status(Status::BadRequest).finalize();
            }
        }

        _ => {
            return response.sized_body(
                Cursor::new(serde_json::to_string(&ErrorMessage { error: "No key found" }).unwrap()))
                .status(Status::BadRequest).finalize();
        }
    }

    generic_create(state, cd, Box::new(Credential::create))
}

#[delete("/<id>")]
pub fn delete_credential(state: State<Conf>, id: u32) -> Response {
    return generic_delete::<Credential>(state, id, Box::new(Credential::delete_by_id));
}

