use iron::prelude::*;
use iron::status;
use iron::Handler;
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use serde_json;
use std::io::Read;
use model::*;
use templates::controller::TemplatesClient;
use router::Router;

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
        let user: User = serde_json::from_str(&new_user).unwrap();
        self.database.deref().lock().unwrap().insert_user(user);

        Ok(Response::with((status::Ok, "Ok")))
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

pub struct DeleteUserHandler {
    database: Arc<Mutex<Database>>,
}
impl DeleteUserHandler {
    pub fn new(db: Arc<Mutex<Database>>) -> DeleteUserHandler {
        DeleteUserHandler{database: db}
    }
}
impl Handler for DeleteUserHandler {
    fn handle(&self, req : &mut Request) -> IronResult<Response> {
        let mut email = String::new();
        req.body.read_to_string(&mut email);
        self.database.deref().lock().unwrap().delete_user(email);

        Ok(Response::with((status::Ok, "Ok")))
    }
}

pub struct GetTemplatesHandler {
    template_client: Arc<Mutex<TemplatesClient>>,
}
impl GetTemplatesHandler {
    pub fn new(client: Arc<Mutex<TemplatesClient>>) -> GetTemplatesHandler {
        GetTemplatesHandler { template_client : client}
    }
}
impl Handler for GetTemplatesHandler {
    fn handle(&self, req : &mut Request) -> IronResult<Response> {
        let templates = self.template_client.deref().lock().unwrap().get_templates();
        Ok(Response::with((status::Ok, templates)))
    }
}

pub struct GetTemplateHandler {
    template_client: Arc<Mutex<TemplatesClient>>,
}
impl GetTemplateHandler {
    pub fn new(client: Arc<Mutex<TemplatesClient>>) -> GetTemplateHandler {
        GetTemplateHandler { template_client : client}
    }
}
impl Handler for GetTemplateHandler {
    fn handle(&self, req : &mut Request) -> IronResult<Response> {
        let ref template_id = req.extensions.get::<Router>().unwrap().find("template_id").unwrap_or("/");
        let template = self.template_client.deref().lock().unwrap().get_template(template_id);
        Ok(Response::with((status::Ok, template)))
    }
}

pub struct EditTemplateHandler {
    template_client: Arc<Mutex<TemplatesClient>>,
}
impl EditTemplateHandler {
    pub fn new(client: Arc<Mutex<TemplatesClient>>) -> EditTemplateHandler {
        EditTemplateHandler { template_client : client}
    }
}
impl Handler for EditTemplateHandler {
    fn handle(&self, req : &mut Request) -> IronResult<Response> {
        let ref template_id = req.extensions.get::<Router>().unwrap().find("template_id").unwrap_or("/");

        let mut template_data = String::new();
        req.body.read_to_string(&mut template_data);
        let data: TemplateData = serde_json::from_str(&template_data).unwrap();

        let updated_template = self.template_client.deref().lock().unwrap().edit_template(template_id, &data.version_id, &data.subject, &data.content);
        Ok(Response::with((status::Ok, updated_template)))
    }
}
#[derive(Serialize, Deserialize)]
struct TemplateData {
    subject: String,
    content: String,
    version_id: String
}