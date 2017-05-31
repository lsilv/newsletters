use hyper::client::Client;
use hyper::header::{Headers, Authorization, ContentType};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

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

}
