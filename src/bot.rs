use std::collections::HashMap;
use chrono::{Utc, SecondsFormat};
use serde_json::json;
use reqwest::blocking::{Client, Response, multipart};
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

    pub fn list_users(&self)->Result<Response, Error>{
        let url = format!("{}://{}/api/v1/users.list", self.protocol, self.base_uri);
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

    pub fn send_file(&self, room: &str, text: &str, description: &str,
                     filepath: &str)->Result<Response, Error>{
        let url = format!("{}://{}/api/v1/rooms.upload/{}",
                          self.protocol, self.base_uri, room);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-type".to_string(), "application/json".to_string());
        headers.insert("X-User-ID".to_string(), self.user_id.to_string());
        headers.insert("X-Auth-Token".to_string(), self.token.to_string());
        let form = multipart::Form::new()
            .text("msg", text.to_string())
            .text("description", description.to_string())
            .file("file", filepath).unwrap();
        post_form(&url, &headers, form)
    }

    pub fn clean_room(&self, room: &str)->Result<Response, Error>{
        let url = format!("{}://{}/api/v1/rooms.cleanHistory", self.protocol, self.base_uri);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-type".to_string(), "application/json".to_string());
        headers.insert("X-User-ID".to_string(), self.user_id.to_string());
        headers.insert("X-Auth-Token".to_string(), self.token.to_string());
        let now = Utc::now();
        let body = json!({
                "roomId": room,
                "oldest": "1970-00-00T00:00:00.000Z",
                "latest": now.to_rfc3339_opts(SecondsFormat::Millis, true),
        });
        post(&url, &headers, Some(serde_json::to_string(&body).unwrap()))
    }

    pub fn invite_user(&self, room: &str, user: &str)->Result<Response, Error>{
        let url = format!("{}://{}/api/v1/channels.invite", self.protocol, self.base_uri);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-type".to_string(), "application/json".to_string());
        headers.insert("X-User-ID".to_string(), self.user_id.to_string());
        headers.insert("X-Auth-Token".to_string(), self.token.to_string());
        let body = json!({
                "roomId": room,
                "userId": user
        });
        post(&url, &headers, Some(serde_json::to_string(&body).unwrap()))
    }

    pub fn create_channel(&self, name: &str, read_only_option: Option<bool>)->Result<Response, Error>{
        let url = format!("{}://{}/api/v1/channels.create", self.protocol, self.base_uri);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-type".to_string(), "application/json".to_string());
        headers.insert("X-User-ID".to_string(), self.user_id.to_string());
        headers.insert("X-Auth-Token".to_string(), self.token.to_string());
        let read_only = match read_only_option{
            Some(value) => value,
            _ => false,
        };
        let body = json!({
                "name": name,
                "readOnly": read_only
        });
        post(&url, &headers, Some(serde_json::to_string(&body).unwrap()))

    }
    pub fn create_user(&self, username: &str, name: &str, email: &str, password: &str, change_password: Option<bool>)->Result<Response, Error>{
        let url = format!("{}://{}/api/v1/users.create", self.protocol, self.base_uri);
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-type".to_string(), "application/json".to_string());
        headers.insert("X-User-ID".to_string(), self.user_id.to_string());
        headers.insert("X-Auth-Token".to_string(), self.token.to_string());
        let require_password_change = match change_password{
            Some(value) => value,
            _ => false,
        };
        let body = json!({
                "username": username,
                "name": name,
                "email": email,
                "password": password,
                "requirePasswordChange": require_password_change
        });
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

fn post_form(url: &str, headers: &HashMap<String, String>,
             form: multipart::Form)->Result<Response, Error>{
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
    client.post(url).multipart(form).send()
}
