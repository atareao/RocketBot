use std::collections::HashMap;
use crate::utils::read_from_toml;

mod utils;

fn main() {
    let options: HashMap<String, String> = read_from_toml(".env");
    let token = options.get("TOKEN").unwrap();
    let id = options.get("ID").unwrap();
    println!("Token: {}, Id: {}", token, id);
}
