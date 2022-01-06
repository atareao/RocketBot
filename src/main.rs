use std::collections::HashMap;
use crate::utils::read_from_toml;
use curl::easy::Easy2;

mod utils;

fn main() {
    let options: HashMap<String, String> = read_from_toml(".env");
    let token = options.get("TOKEN").unwrap();
    let id = options.get("ID").unwrap();
    println!("Token: {}, Id: {}", token, id);
    let handle = Easy2::new();
    handle.header_function(|header|{
        print!("X-Auth-Token: {}", token);
        print!("X-User-Id: {}", id);
        print!("Content-type", );
    })

}
