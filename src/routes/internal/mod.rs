use std::io::Cursor;

use rocket::{Response, State};
use rocket::http::{ContentType, Status};
use rocket::response::ResponseBuilder;
use rocket_contrib::json::Json;
use rusqlite::{Connection, Error, Result as SR};

use crate::Conf;
use crate::datastructures::{ErrorMessage, IdObj, ProjectData};

pub fn generic_get<'a, T: ProjectData<'a>>(state: State<Conf>,
                                           limit: Option<u32>,
                                           offset: Option<u32>,
                                           mapping: Box<dyn Fn(&Connection, Option<u32>, Option<u32>) -> SR<Vec<T>>>,
) -> Response<'static> {
    let conn = state.get_new_db_connection();
    let mut response = json_response();
    match mapping(&conn, limit, offset) {
        Ok(vec) => {
            response.sized_body(Cursor::new(serde_json::to_string(&vec).unwrap()))
                .status(Status::Ok);
        }
        _ => {
            response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Error" }).unwrap()))
                .status(Status::InternalServerError);
        }
    }
    response.finalize()
}


pub fn generic_get_by_id<'a, T: ProjectData<'a>>(state: State<Conf>,
                                                 id: u32,
                                                 mapping: Box<dyn Fn(&Connection, u32) -> SR<Option<T>>>,
) -> Response<'static> {
    let conn = state.get_new_db_connection();
    let mut response = json_response();
    match mapping(&conn, id) {
        Ok(Some(ref s)) => {
            response.sized_body(Cursor::new(serde_json::to_string(s).unwrap()))
                .status(Status::Ok);
        }
        Err(Error::QueryReturnedNoRows) => {
            response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Id not found" }).unwrap()))
                .status(Status::NotFound);
        }
        _ => {
            response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Error" }).unwrap()))
                .status(Status::InternalServerError);
        }
    }
    response.finalize()
}

pub fn generic_create<'a, T: ProjectData<'a>>(state: State<Conf>,
                                              data: Json<T>,
                                              mapping: Box<dyn Fn(&Connection, &T) -> SR<u32>>,
) -> Response {
    let conn = state.get_new_db_connection();
    let mut response = json_response();
    match mapping(&conn, &data) {
        Ok(i) => {
            response.sized_body(Cursor::new(serde_json::to_string(&data.new_with_new_id(i)
            ).unwrap()))
                .status(Status::Created);
        }
        Err(_) => {
            response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Error creating object" }).unwrap()))
                .status(Status::InternalServerError);
        }
    }
    response.finalize()
}

pub fn generic_update<'a, T: ProjectData<'a>>(state: State<Conf>,
                                              data: Json<T>,
                                              mapping: Box<dyn Fn(&Connection, &T) -> SR<()>>,
) -> Response {
    let conn = state.get_new_db_connection();
    let mut response = json_response();
    match mapping(&conn, &data) {
        Ok(()) => {
            response.sized_body(Cursor::new(serde_json::to_string(&data.0).unwrap()))
                .status(Status::Ok);
        }
        _ => {
            response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Error" }).unwrap()))
                .status(Status::InternalServerError);
        }
    }
    response.finalize()
}

pub fn generic_delete<'a, T: ProjectData<'a>>(state: State<Conf>,
                                              id: u32,
                                              mapping: Box<dyn Fn(&Connection, u32) -> SR<()>>,
) -> Response {
    let conn = state.get_new_db_connection();
    let mut response = json_response();
    match mapping(&conn, id) {
        Ok(()) => {
            response.sized_body(Cursor::new(serde_json::to_string(&IdObj { id: Some(id) }).unwrap()))
                .status(Status::Ok);
        }
        Err(Error::QueryReturnedNoRows) => {
            response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Id not found" }).unwrap()))
                .status(Status::NotFound);
        }
        _ => {
            response.sized_body(Cursor::new(serde_json::to_string(&ErrorMessage { error: "Error" }).unwrap()))
                .status(Status::InternalServerError);
        }
    }
    response.finalize()
}

pub fn json_response<'a>() -> ResponseBuilder<'a> {
    let mut response = Response::build();
    response.header(ContentType::JSON);
    response
}