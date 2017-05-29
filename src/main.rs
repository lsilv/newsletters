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
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hyper;
extern crate hyper_native_tls;

use iron::prelude::*;
use router::Router;
use iron::status;
use std::path::Path;
use std::sync::{Arc, Mutex};
use model::*;
use controller::*;
use hyper::client::Client;
use hyper::header::{Headers, Authorization, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("ui/index.html"))))
}

fn jshandler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("ui/index.js"))))
}

//TODO: validations
//TODO: use hyper instead of iron

fn main() {
    env_logger::init().unwrap();

    sendMail("silvialeontiuc@gmail.com");

    /*let database = Database::new("postgres", "", "172.18.10.25", "users");
    let database_arc = Arc::new(Mutex::new(database));

    let mut router = Router::new();

    router.get("", handler, "index");
    router.get("/index.js", jshandler, "jsindex");

    router.post("/users", AddUserHandler::new(database_arc.clone()), "Add user in DB");
    router.get("/users", GetUsersHandler::new(database_arc.clone()), "Get all users from DB");
    router.delete("/users", DeleteUserHandler::new(database_arc.clone()), "Delete user by email from DB");

    Iron::new(router).http("localhost:3000").unwrap();*/
}

#[derive(Serialize, Deserialize)]
struct Email {
    email: String
}
#[derive(Serialize, Deserialize)]
struct ToEntity {
    to: Vec<Email>,
    subject: String
}
#[derive(Serialize, Deserialize)]
struct FromEntity {
    email: String,
    name: String
}
#[derive(Serialize, Deserialize)]
struct Content {
    value: String,
    #[serde(rename = "type")]
    content_type: String
}
#[derive(Serialize, Deserialize)]
struct SandBoxMode {
    enable: bool
}
#[derive(Serialize, Deserialize)]
struct TestingMode {
    sandbox_mode: SandBoxMode
}
#[derive(Serialize, Deserialize)]
struct SendGridRequest {
    personalizations: Vec<ToEntity>,
    from: FromEntity,
    content: Vec<Content>,
    mail_settings: TestingMode
}

fn buildRequest(send_to: &str) -> SendGridRequest {
    let to = vec![Email{email: send_to.to_string()}];
    let to_entity = ToEntity{ to : to, subject: "subject".to_string()};
    let from_entity = FromEntity { name : "From Work".to_string(), email : "test@test".to_string()};
    let content = Content { value : "Hello, me".to_string(), content_type : "text/plain".to_string() };
    SendGridRequest{ personalizations: vec![to_entity], from : from_entity, content : vec![content], mail_settings : TestingMode{ sandbox_mode : SandBoxMode {enable: true}} }
}

fn sendMail(send_to: &str) {

    let body = json!(buildRequest(send_to)).to_string();

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set(Authorization("Bearer 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".to_owned()));
    let res = client.
        post("https://api.sendgrid.com/v3/mail/send").
        headers(headers).
        body(&body).
        send().
        unwrap();
    println!("{}", res.status);
}
