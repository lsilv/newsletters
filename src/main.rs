mod users;
mod templates;
mod emails;

extern crate iron;
extern crate router;
extern crate postgres;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate hyper_native_tls;

use iron::prelude::*;
use router::Router;
use iron::status;
use std::path::Path;
use std::sync::{Arc, Mutex};

use users::database::*;
use users::handlers::*;
use templates::templates_client::TemplatesClient;
use templates::handlers::*;
use emails::*;

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("ui/index.html"))))
}

fn usersjs_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("ui/users.js"))))
}
fn templatesjs_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("ui/templates.js"))))
}

//TODO: exception handling
//TODO: validations
//TODO: use hyper instead of iron ?

fn main() {
    env_logger::init();

    let database = Database::new("postgres", "", "172.18.10.25", "users");
    let database_arc = Arc::new(Mutex::new(database));

    let mut router = Router::new();

    router.get("", handler, "index");
    router.get("/users.js", usersjs_handler, "Return user.js from server");
    router.get("/templates.js", templatesjs_handler, "Return templates.js from server");

    router.post("/users", AddUserHandler::new(database_arc.clone()), "Add user in DB");
    router.get("/users", GetUsersHandler::new(database_arc.clone()), "Get all users from DB");
    router.delete("/users", DeleteUserHandler::new(database_arc.clone()), "Delete user by email from DB");

    let templates_client_arc = Arc::new(Mutex::new(TemplatesClient::new()));

    router.get("/templates", GetTemplatesHandler::new(templates_client_arc.clone()), "Get all templates");
    router.post("/templates", AddTemplateHandler::new(templates_client_arc.clone()), "Add a new template");
    router.get("/templates/:template_id", GetTemplateHandler::new(templates_client_arc.clone()), "Get template by id");
    router.put("/templates/:template_id", EditTemplateHandler::new(templates_client_arc.clone()), "Edit template");
    router.delete("/templates/:template_id/:version_id", DeleteTemplateHandler::new(templates_client_arc.clone()), "Delete template");

    router.post("/send/:template_id", MailSender::new(database_arc.clone()), "Send mail to all users from DB");

    Iron::new(router).http("localhost:3000").unwrap();
}
