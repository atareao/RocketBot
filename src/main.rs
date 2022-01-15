mod bot;
mod utils;

use crate::utils::read_from_toml;
use crate::bot::Bot;
use clap::{App, Arg, AppSettings};

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str =env!("CARGO_PKG_VERSION");
const AUTHORS: &str =env!("CARGO_PKG_AUTHORS");

fn main() {
    let config = read_from_toml(".env");
    let protocol = config.get("PROTOCOL").unwrap();
    let base_uri = config.get("BASE_URI").unwrap();
    let user_id = config.get("USER_ID").unwrap();
    let token = config.get("ACCESS_TOKEN").unwrap();
    let bot = Bot::new(protocol, base_uri, user_id, token);
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::new("debug")
             .short('d')
             .long("debug")
             .takes_value(false))
        .subcommand(App::new("list")
                    .about("List")
                    .subcommand(App::new("rooms")
                                .about("List rooms")
                                )
                    )
        .subcommand(App::new("send")
                    .about("Send")
                    .subcommand(App::new("message")
                                .about("Send message")
                                .arg(Arg::new("room")
                                     .short('r')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("text")
                                     .short('t')
                                     .required(true)
                                     .takes_value(true))
                                )
                    .subcommand(App::new("image")
                                .about("Send image")
                                )
                    )
        .get_matches();
    if let Some(sub) = matches.subcommand_matches("send"){
        if let Some(subsub) = sub.subcommand_matches("message"){
            let room = subsub.value_of("room").unwrap();
            let text = subsub.value_of("text").unwrap();
            match bot.send_message(room, text){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }
    }else if let Some(sub) = matches.subcommand_matches("list"){
        if let Some(subsub) = sub.subcommand_matches("rooms"){
            match bot.list_rooms(){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }
    }
}
