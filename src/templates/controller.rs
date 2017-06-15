use hyper::client::Client;
use hyper::header::{Headers, Authorization, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;
use serde_json;

#[derive(Serialize, Deserialize)]
struct ReceivedTemplateData {
    subject: String,
    content: String,
    version_id: String
}
#[derive(Serialize, Deserialize)]
struct SentTemplateData {
    subject: String,
    plain_content: String
}

pub struct TemplatesClient {
    api_client : Client,
    headers: Headers,
    url: String
}

impl TemplatesClient {

    pub fn new() -> TemplatesClient {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let api_client = Client::with_connector(connector);

        let mut headers = Headers::new();
        headers.set(ContentType::json());
        headers.set(Authorization("Bearer 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".to_owned()));

        TemplatesClient { api_client : api_client, headers : headers, url : "https://api.sendgrid.com/v3/templates".to_string()}
    }

    pub fn get_templates(&self) -> String {
        let mut response = self.api_client.
            get(&self.url).
            headers(self.headers.clone()).
            send().
            unwrap();
        let mut response_body = String::new();
        response.read_to_string(&mut response_body);
        response_body
    }

    pub fn get_template(&self, id : &str) -> String {
        let url = self.url.clone() + "/" + id;
        let mut response = self.api_client.
            get(&url).
            headers(self.headers.clone()).
            send().
            unwrap();
        let mut response_body = String::new();
        response.read_to_string(&mut response_body);
        response_body
    }

    pub fn edit_template(&self, template_id: &str, received_data: &str) -> String {

        let received_data: ReceivedTemplateData = serde_json::from_str(received_data).unwrap();
        let send_data = SentTemplateData { subject: received_data.subject, plain_content : received_data.content };

        let url = self.url.clone() + "/" + template_id + "/versions/" + &received_data.version_id;

        let mut response = self.api_client.
            patch(&url).
            headers(self.headers.clone()).
            body(&json!(send_data).to_string()).
            send().
            unwrap();
        let mut response_body = String::new();
        response.read_to_string(&mut response_body);
        response_body
    }

    pub fn add_template(&self, name: &str) -> String {

        let url = self.url.clone();

        let mut response = self.api_client.
            post(&url).
            headers(self.headers.clone()).
            body(name).
            send().
            unwrap();
        let mut response_body = String::new();
        response.read_to_string(&mut response_body);
        response_body
    }

}
