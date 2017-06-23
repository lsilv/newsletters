use hyper::client::Client;
use hyper::header::{Headers, Authorization, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

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

fn build_request(send_to: &str) -> SendGridRequest {
    let to = vec![Email{email: send_to.to_string()}];
    let to_entity = ToEntity{ to : to, subject: "subject".to_string()};
    let from_entity = FromEntity { name : "From Work".to_string(), email : "test@test".to_string()};
    let content = Content { value : "Hello, me".to_string(), content_type : "text/plain".to_string() };
    SendGridRequest{ personalizations: vec![to_entity], from : from_entity, content : vec![content], mail_settings : TestingMode{ sandbox_mode : SandBoxMode {enable: true}} }
}

pub fn send_mail(send_to: &str) {

    let body = json!(build_request(send_to)).to_string();

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
