use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};


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

