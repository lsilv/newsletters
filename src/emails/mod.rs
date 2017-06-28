use hyper::client::Client;
use hyper::header::{Headers, Authorization, ContentType};
use hyper::net::HttpsConnector;
use hyper::status::StatusCode;
use hyper_native_tls::NativeTlsClient;
use iron::prelude::*;
use iron::Handler;
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use router::Router;
use users::database::*;
use postgres::rows::Rows;

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
    template_id: String,
    mail_settings: TestingMode
}


pub struct MailSender {
    database: Arc<Mutex<Database>>,
}

impl MailSender {
    pub fn new(db: Arc<Mutex<Database>>) -> MailSender {
        MailSender { database: db }
    }

    fn build_request(&self, template_id: &str, send_to: &Rows<>) -> SendGridRequest {
        let mut to = vec![];
        for row in send_to {
            to.push(Email { email: row.get("email") });
        }
        let to_entity = ToEntity { to: to, subject: "subject".to_string() };
        let from_entity = FromEntity { name: "From Work".to_string(), email: "test@test".to_string() };
        SendGridRequest { personalizations: vec![to_entity], from: from_entity, template_id: template_id.to_string(), mail_settings: TestingMode { sandbox_mode: SandBoxMode { enable: true } } }
    }

    fn send_mail(&self, template_id: &str, send_to: &Rows<>) -> StatusCode {
        let body = json!(self.build_request(template_id, send_to)).to_string();
        println!("{}", body);

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
        println!("Mail sent: {}", res.status);
        res.status
    }
}

impl Handler for MailSender {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref template_id = req.extensions.get::<Router>().unwrap().find("template_id").unwrap_or("/");

        let database = self.database.deref().lock().unwrap();
        let response = self.send_mail(template_id, &database.get_users());

        Ok(Response::with(response))
    }
}
