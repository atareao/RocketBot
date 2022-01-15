mod bot;
mod utils;

use crate::utils::read_from_toml;
use crate::bot::Bot;
use clap::{App, Arg, AppSettings};
use dirs::config_dir;

const NAME: &str =env!("CARGO_PKG_NAME");
const DESCRIPTION: &str =env!("CARGO_PKG_DESCRIPTION");
const VERSION: &str =env!("CARGO_PKG_VERSION");
const AUTHORS: &str =env!("CARGO_PKG_AUTHORS");

fn main() {
    let config_path = config_dir().unwrap()
        .join("rocketbot")
        .join("rocketbot.conf");
    if !config_path.exists(){
        println!("Configure Rocket Bot");
        return;
    }
    let config = read_from_toml(config_path.to_str().unwrap());
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
                    .subcommand(App::new("users")
                                .about("List users")
                                )
                    )
        .subcommand(App::new("clean")
                    .about("Clean")
                    .subcommand(App::new("room")
                                .about("Clean room")
                                .arg(Arg::new("room")
                                     .short('r')
                                     .required(true)
                                     .takes_value(true))
                                )
                    )
        .subcommand(App::new("invite")
                    .about("Invite")
                    .subcommand(App::new("user")
                                .about("Invite usert to room")
                                .arg(Arg::new("room")
                                     .short('r')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("user")
                                     .short('u')
                                     .required(true)
                                     .takes_value(true))
                                )
                    )
        .subcommand(App::new("create")
                    .about("Create")
                    .subcommand(App::new("user")
                                .about("Create new user")
                                .arg(Arg::new("username")
                                     .short('u')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("name")
                                     .short('n')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("email")
                                     .short('e')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("password")
                                     .short('p')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("required_password_change")
                                     .short('r')
                                     .required(false)
                                     .takes_value(false))
                                )
                    .subcommand(App::new("channel")
                                .about("Create new channel")
                                .arg(Arg::new("name")
                                     .short('n')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("read_only")
                                     .short('r')
                                     .required(false)
                                     .takes_value(false))
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
                    .subcommand(App::new("file")
                                .about("Send file")
                                .arg(Arg::new("room")
                                     .short('r')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("text")
                                     .short('t')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("description")
                                     .short('d')
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::new("filepath")
                                     .short('f')
                                     .required(true)
                                     .takes_value(true))
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
        }else if let Some(subsub) = sub.subcommand_matches("file"){
            let room = subsub.value_of("room").unwrap();
            let text = subsub.value_of("text").unwrap();
            let description = subsub.value_of("description").unwrap();
            let filepath = subsub.value_of("filepath").unwrap();
            match bot.send_file(room, text, description, filepath){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }
    }else if let Some(sub) = matches.subcommand_matches("invite"){
        if let Some(subsub) = sub.subcommand_matches("user"){
            let room = subsub.value_of("room").unwrap();
            let user = subsub.value_of("user").unwrap();
            match bot.invite_user(room, user){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }
    }else if let Some(sub) = matches.subcommand_matches("clean"){
        if let Some(subsub) = sub.subcommand_matches("room"){
            let room = subsub.value_of("room").unwrap();
            match bot.clean_room(room){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }
    }else if let Some(sub) = matches.subcommand_matches("list"){
        if let Some(_subsub) = sub.subcommand_matches("rooms"){
            match bot.list_rooms(){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }else if let Some(_subsub) = sub.subcommand_matches("users"){
            match bot.list_users(){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }
    }else if let Some(sub) = matches.subcommand_matches("create"){
        if let Some(subsub) = sub.subcommand_matches("user"){
            let username = subsub.value_of("username").unwrap();
            let name = subsub.value_of("name").unwrap();
            let email = subsub.value_of("email").unwrap();
            let password = subsub.value_of("password").unwrap();
            let required_password_change = subsub.is_present("required_password_change");
            match bot.create_user(username, name, email, password, Some(required_password_change)){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }else if let Some(subsub) = sub.subcommand_matches("channel"){
            let name = subsub.value_of("name").unwrap();
            let read_only = subsub.is_present("read_only");
            match bot.create_channel(name, Some(read_only)){
                Ok(result) => println!("{}", result.text().unwrap()),
                Err(result) => println!("{}", result.to_string())
            }
        }
    }
}
