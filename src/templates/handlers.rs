use iron::prelude::*;
use iron::status;
use iron::Handler;
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::io::Read;
use router::Router;
use templates::templates_client::TemplatesClient;

pub struct GetTemplatesHandler {
    template_client: Arc<Mutex<TemplatesClient>>,
}
impl GetTemplatesHandler {
    pub fn new(client: Arc<Mutex<TemplatesClient>>) -> GetTemplatesHandler {
        GetTemplatesHandler { template_client : client}
    }
}
impl Handler for GetTemplatesHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
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

        let updated_template = self.template_client.deref().lock().unwrap().edit_template(template_id, &template_data);
        Ok(Response::with((status::Ok, updated_template)))
    }
}

pub struct AddTemplateHandler {
    template_client: Arc<Mutex<TemplatesClient>>,
}
impl AddTemplateHandler {
    pub fn new(client: Arc<Mutex<TemplatesClient>>) -> AddTemplateHandler {
        AddTemplateHandler { template_client : client}
    }
}
impl Handler for AddTemplateHandler {
    fn handle(&self, req : &mut Request) -> IronResult<Response> {

        let mut template_data = String::new();
        req.body.read_to_string(&mut template_data);

        let new_template = self.template_client.deref().lock().unwrap().add_template(&template_data);
        Ok(Response::with((status::Ok, new_template)))
    }
}

pub struct DeleteTemplateHandler {
    template_client: Arc<Mutex<TemplatesClient>>,
}
impl DeleteTemplateHandler {
    pub fn new(client: Arc<Mutex<TemplatesClient>>) -> DeleteTemplateHandler {
        DeleteTemplateHandler { template_client : client}
    }
}
impl Handler for DeleteTemplateHandler {
    fn handle(&self, req : &mut Request) -> IronResult<Response> {
        let ref template_id = req.extensions.get::<Router>().unwrap().find("template_id").unwrap_or("/");
        let ref version_id = req.extensions.get::<Router>().unwrap().find("version_id").unwrap_or("/");

        let new_template = self.template_client.deref().lock().unwrap().delete_template(template_id, version_id);
        Ok(Response::with((status::Ok, new_template)))
    }
}
