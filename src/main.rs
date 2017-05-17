mod model;
mod controller;

extern crate iron;
extern crate router;
extern crate postgres;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use iron::prelude::*;
use router::Router;
use iron::status;
use std::path::Path;
use std::sync::{Arc, Mutex};
use model::*;
use controller::*;

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("ui/index.html"))))
}

fn jshandler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("ui/index.js"))))
}


fn main() {
    let database = Database::new("postgres", "postgres", "localhost", "users");
    let database_arc = Arc::new(Mutex::new(database));

    let mut router = Router::new();

    router.get("/index", handler, "index");
    router.get("/index.js", jshandler, "jsindex");

    router.post("/users", AddUserHandler::new(database_arc.clone()), "Add user in DB");
    router.get("/users", GetUsersHandler::new(database_arc.clone()), "Get all users from DB");

    Iron::new(router).http("localhost:3000").unwrap();
}
