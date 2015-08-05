extern crate hipchat_client;
extern crate hyper;
extern crate rustc_serialize;

use std::io::prelude::*;
use std::fs::File;
use hipchat_client::client::Client as HipchatClient;
//use hipchat_client::room::RoomsRequest;
use rustc_serialize::json;

#[derive(RustcDecodable)]
struct Config {
    token: String,
    origin: String,
    room: String
}

fn setup() -> (HipchatClient, Config) {
    let contents = &mut String::new();
    File::open("./settings.json")
        .unwrap_or_else(|e| panic!("{}", e))
        .read_to_string(contents)
        .unwrap_or_else(|e| panic!("{}", e));
    let config = json::decode::<Config>(contents)
        .unwrap_or_else(|e| panic!("{}", e));
    (HipchatClient::new(config.origin.clone(), config.token.clone()), config)
}

#[test]
fn integration_get_room() {
    let (client, config) = setup();
    let room = client.get_room(&config.room);
    println!("{:#?}", room);
    assert_eq!(room.name, config.room);
}

#[test]
fn integration_get_rooms() {
    let (client, config) = setup();
    let rooms = client.get_rooms(None);
    println!("{:#?}", rooms);
    assert!(rooms.items.iter().any(|r| r.name == config.room));
}

