use rocket_contrib::json::Json;
use rocket::http::{Status, ContentType};

use crate::datastructures::{Schema, ErrorMessage};
use crate::Conf;
use rocket::{State, Response};
use crate::dao::ConnectionRestMapping;
use std::io::Cursor;

use super::internal::*;

#[get("/?<limit>&<offset>")]
pub fn get_schemas(state: State<Conf>, limit: Option<u32>, offset: Option<u32>) -> Response {
    return generic_get(state, limit, offset, Box::new(Schema::get_all));
}

#[get("/<id>")]
pub fn get_schema_by_id(state: State<Conf>, id: u32) -> Response<'static> {
    return generic_get_by_id(state, id, Box::new(Schema::get_by_id));
}

#[post("/", data = "<schema>")]
pub fn create_schema(state: State<Conf>, schema: Json<Schema>) -> Response {
    if schema.schema.is_none() {
        let mut response = Response::build();
        response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Invalid schema field" }).unwrap()))
            .status(Status::InternalServerError);
        response.header(ContentType::JSON).finalize()
    } else {
        generic_create(state, schema, Box::new(Schema::create))
    }
}

#[put("/", data = "<schema>")]
pub fn update_schema(state: State<Conf>, schema: Json<Schema>) -> Response {
    if schema.schema.is_some() && schema.id.is_some() {
        generic_update(state, schema, Box::new(Schema::update))
    } else {
        let mut response = Response::build();
        response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Invalid request body" }).unwrap()))
            .status(Status::UnprocessableEntity);
        response.header(ContentType::JSON).finalize()
    }
}

#[delete("/<id>")]
pub fn delete_schema(state: State<Conf>, id: u32) -> Response {
    return generic_delete::<Schema>(state, id, Box::new(Schema::delete_by_id));
}
