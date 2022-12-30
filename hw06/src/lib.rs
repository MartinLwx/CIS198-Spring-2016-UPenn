use rustc_serialize::json;
use std::io::Read;

use hyper::net::HttpsConnector;
use hyper::status::StatusCode;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;

pub const SERVER_ADDR: &'static str = "127.0.0.1:1980";
pub const BOT_ADDR: &'static str = "127.0.0.1:1981";
pub const HTML_ADDR: &'static str = "http://127.0.0.1:1980";

pub const HTML_DATA: &'static str = "data/index.html";
pub const HTML_HEADER: &'static str = "html/header.html";
pub const HTML_FOOTER: &'static str = "html/footer.html";

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Message {
    pub user: String,
    pub text: String,
}

impl Message {
    pub fn new(user: String, text: String) -> Message {
        Message { text, user }
    }
}

pub struct UserClient {
    username: String,
    server_addr: String,
    client: hyper::Client,
}

impl UserClient {
    pub fn new(username: String, server_addr: String) -> UserClient {
        UserClient {
            username,
            server_addr,
            client: hyper::Client::new(),
        }
    }

    // TODO: Implement send_msg
    pub fn send_msg(&self, msg: Message) -> hyper::Result<(StatusCode, String)> {
        let mut response = self
            .client
            .post(&self.server_addr)
            .body(json::encode(&msg).unwrap().as_bytes())
            .send()?;
        let mut buf = String::new();
        response.read_to_string(&mut buf).unwrap();
        Ok((response.status, buf))
    }

    pub fn get_content(&self) -> hyper::Result<(StatusCode, String)> {
        let mut response = self.client.get(&self.server_addr).send()?;
        let mut buf = String::new();
        response.read_to_string(&mut buf).unwrap();
        Ok((response.status, buf))
    }
}
