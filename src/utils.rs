use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use curl::easy::{Easy2, Handler, WriteError};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn post(url: &str, headers: &HashMap<String, String>)->Result<String, String>{
    let mut header_map = HeaderMap::new();
    for keyvalue in headers{
        header_map.insert(HeaderName::from_str(keyvalue.0).unwrap(),
                          HeaderValue::from_str(keyvalue.1).unwrap());
    }
    let client = Client::builder()
        .default_headers(header_map)
        .build()
        .unwrap();
    match client.post(url).send(){
        Ok()


    }

}

pub fn get(url: &str)->Result<String, &'static str>{
    let mut handle = Easy2::new(Collector(Vec::new()));
    handle.get(true).unwrap();
    handle.url(url).unwrap();
    match handle.response_code(){
        Ok(200) => {
            let contents = handle.get_ref();
            Ok(String::from_utf8_lossy(&contents.0).to_string())
        }
        Err(e) => Err(&e.to_string())
    }
}


pub fn read_from_toml(filename: &str)->HashMap<String, String>{
    let mut options:HashMap<String, String> = HashMap::new();
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines {
        let keyvalue = line.unwrap();
        let v: Vec<&str> = keyvalue.split('=').collect();
        let key = v[0].trim().to_string();
        let value = v[1].trim().to_string();
        options.insert(key, value);
    }
    options
}

pub fn read_from_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

