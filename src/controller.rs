use iron::prelude::*;
use iron::status;
use iron::Handler;
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use serde_json;
use std::io::Read;
use model::*;

pub struct AddUserHandler {
    database: Arc<Mutex<Database>>,
}
impl AddUserHandler {
    pub fn new(db: Arc<Mutex<Database>>) -> AddUserHandler {
        AddUserHandler{database: db}
    }
}
impl Handler for AddUserHandler {
    fn handle(&self, req : &mut Request) -> IronResult<Response> {
        let mut new_user = String::new();
        req.body.read_to_string(&mut new_user);
        println!("{}", new_user);

        let user: User = serde_json::from_str(&new_user).unwrap();

//        let ujson = json!(u);
        Ok(Response::with((status::Ok, new_user)))
    }
}

pub struct GetUsersHandler {
    database: Arc<Mutex<Database>>,
}
impl GetUsersHandler {
    pub fn new(db: Arc<Mutex<Database>>) -> GetUsersHandler {
        GetUsersHandler{database: db}
    }
}
impl Handler for GetUsersHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut users = Vec::new();
        let database = self.database.deref().lock().unwrap();
        for row in &database.get_users() {
            let user = User{email : row.get("email"), first_name: row.get("first_name"), last_name: row.get("last_name")};
            users.push(user);
        }
        Ok(Response::with((status::Ok, json!(users).to_string())))
    }
}
