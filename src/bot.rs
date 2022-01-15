use std::collections::HashMap;
use serde_json::json;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use reqwest::Error;
use std::str::FromStr;

pub struct Bot{
    protocol: String,
    base_uri: String,
    user_id: String,
    token: String,
}

impl Bot{
    pub fn new(protocol: &str, base_uri: &str, user_id: &str, token: &str) -> Bot{
        Self {
            protocol: protocol.to_string(),
            base_uri: base_uri.to_string(),
            user_id: user_id.to_string(),
            token: token.to_string(),
        }
    }

    pub fn list_rooms(&self)->Result<Response, Error>{
        let url = format!("{}://{}/api/v1/rooms.get", self.protocol, self.base_uri);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-type".to_string(), "application/json".to_string());
        headers.insert("X-User-ID".to_string(), self.user_id.to_string());
        headers.insert("X-Auth-Token".to_string(), self.token.to_string());
        get(&url, &headers)

    }

    pub fn send_message(&self, room: &str, text: &str)->Result<Response, Error>{
        let url = format!("{}://{}/api/v1/chat.sendMessage", self.protocol, self.base_uri);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-type".to_string(), "application/json".to_string());
        headers.insert("X-User-ID".to_string(), self.user_id.to_string());
        headers.insert("X-Auth-Token".to_string(), self.token.to_string());
        let body = json!({
            "message": {
                "rid": room,
                "msg": text
        }});
        post(&url, &headers, Some(serde_json::to_string(&body).unwrap()))
    }
}

fn get(url: &str, headers: &HashMap<String, String>)->Result<Response, Error>{
    println!("URL: {}", url);
    let mut header_map = HeaderMap::new();
    for keyvalue in headers{
        header_map.insert(HeaderName::from_str(keyvalue.0).unwrap(),
                          HeaderValue::from_str(keyvalue.1).unwrap());
    }
    let client = Client::builder()
        .default_headers(header_map)
        .build()
        .unwrap();
    client.get(url).send()
}

fn post(url: &str, headers: &HashMap<String, String>, body: Option<String>)->Result<Response, Error>{
    println!("URL: {}", url);
    let mut header_map = HeaderMap::new();
    for keyvalue in headers{
        header_map.insert(HeaderName::from_str(keyvalue.0).unwrap(),
                          HeaderValue::from_str(keyvalue.1).unwrap());
    }
    let client = Client::builder()
        .default_headers(header_map)
        .build()
        .unwrap();
    match body{
        Some(content) => {
            println!("The content: {}", content);
            client.post(url).body(content).send()},
        None => client.post(url).send(),
    }
}
