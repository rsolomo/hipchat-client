use std::io::prelude::*;
use hyper::Client as HyperClient;
use hyper::header::{Authorization, Bearer, Location};
use hyper::status::StatusClass;
use emoticon::Emoticon;
use room::{RoomDetail, RoomUpdate, Rooms, RoomsRequest};
use rustc_serialize::json;
use rustc_serialize::json::Json;
use url::{form_urlencoded, Url};

pub struct Client {
    base_url: String,
    auth: Authorization<Bearer>,
    hyper_client: HyperClient
}

impl Client {
    /// Creates a new HipChat client
    pub fn new<T: Into<String>, O: AsRef<str>>(origin: O, token: T) -> Self {
        Client {
            base_url: format!("{}/v2", origin.as_ref()),
            auth: Authorization(Bearer { token: token.into() }),
            hyper_client: HyperClient::new()
        }
    }

    pub fn get_emoticon<T: AsRef<str>>(&self, emoticon_id_or_shortcut: T) -> Emoticon {
        let mut res = self.hyper_client.get(&format!("{}/emoticon/{}", self.base_url, emoticon_id_or_shortcut.as_ref()))
            .header(self.auth.to_owned())
            .send()
            .unwrap();

        if res.status.class() != StatusClass::Success {
            panic!("{}", res.status);
        }

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        json::decode(&body).unwrap()
    }
    /// Retrieves details of a room
    pub fn get_room<T: AsRef<str>>(&self, room_id_or_name: T) -> RoomDetail {
        let mut res = self.hyper_client.get(&format!("{}/room/{}", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .send()
            .unwrap();

        if res.status.class() != StatusClass::Success {
            panic!("{}", res.status);
        }

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        json::decode(&body).unwrap()
    }
    /// Updates a room
    pub fn update_room<T: AsRef<str>>(&self, room_id_or_name: T, req: &RoomUpdate) -> RoomDetail {
        let body = json::encode(req).unwrap();
        let mut res = self.hyper_client.put(&format!("{}/room/{}", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .body(body.as_bytes())
            .send()
            .unwrap();

        if res.status.class() != StatusClass::Success {
            panic!("{}", res.status);
        }

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        json::decode(&body).unwrap()
    }
    /// Deletes a room
    pub fn delete_room<T: AsRef<str>>(&self, room_id_or_name: T) -> () {
        let res = self.hyper_client.delete(&format!("{}/room/{}", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .send()
            .unwrap();

        if res.status.class() != StatusClass::Success {
            panic!("{}", res.status);
        }
    }
    /// Retrieves a list of rooms
    pub fn get_rooms(&self, req: Option<&RoomsRequest>) -> Rooms {
        let mut pairs = Vec::new();
        if let Some(rooms_request) = req {
            if let Some(start_index) = rooms_request.start_index {
                pairs.push(("start-index", start_index.to_string()));
            }
            if let Some(max_results) = rooms_request.max_results {
                pairs.push(("max-results", max_results.to_string()));
            }
            if let Some(include_private) = rooms_request.include_private {
                pairs.push(("include-private", include_private.to_string()));
            }
            if let Some(include_archived) = rooms_request.include_private {
                pairs.push(("include-archived", include_archived.to_string()));
            }
        }

        let mut url = Url::parse(&format!("{}/room", self.base_url)).unwrap();
        url.query = Some(form_urlencoded::serialize(pairs));

        let mut res = self.hyper_client.get(url)
            .header(self.auth.to_owned())
            .send()
            .unwrap();

        if res.status.class() != StatusClass::Success {
            panic!("{}", res.status);
        }

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        json::decode(&body).unwrap()
    }
    /// Retrieves a room's avatar
    pub fn get_room_avatar<T: AsRef<str>>(&self, room_id_or_name: T) -> String {
        let res = self.hyper_client.get(&format!("{}/room/{}/avatar", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .send()
            .unwrap();

        match res.headers.get::<Location>() {
            Some(location) => location.0.to_owned(),
            None => panic!()
        }
    }
    pub fn update_room_avatar<T: AsRef<str>, U: Into<String>>(&self, room_id_or_name: T, avatar: U) -> () {
        let mut obj = json::Object::new();
        obj.insert("avatar".to_owned(), Json::String(avatar.into()));
        let body = Json::Object(obj).to_string();

        let res = self.hyper_client.put(&format!("{}/room/{}", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .body(body.as_bytes())
            .send()
            .unwrap();

        if res.status.class() != StatusClass::Success {
            panic!("{}", res.status);
        }

    }
    /// Delete a room's avatar
    pub fn delete_room_avatar<T: AsRef<str>>(&self, room_id_or_name: T) {
        let res = self.hyper_client.get(&format!("{}/room/{}/avatar", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .send()
            .unwrap();

        if res.status.class() != StatusClass::Success {
            panic!("{}", res.status);
        }
    }
}
