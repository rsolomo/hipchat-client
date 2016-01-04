use std::io::prelude::*;
use std::collections::BTreeMap;
use std::time::Duration;

use hyper::Client as HyperClient;
use hyper::header::{Authorization, Bearer, ContentType};
use hyper::status::StatusClass;
use rustc_serialize::json::{self, Json};
use url::{form_urlencoded, Url};

use emoticon::Emoticon;
use error::Error;
use room::{RoomDetail, RoomMessage, RoomUpdate, Rooms, RoomsRequest, Notification};

pub struct Client {
    base_url: String,
    auth: Authorization<Bearer>,
    hyper_client: HyperClient
}

impl Client {
    /// Creates a new HipChat API v2 client
    pub fn new<T: Into<String>, O: AsRef<str>>(origin: O, token: T) -> Self {
        Client {
            base_url: format!("{}/v2", origin.as_ref()),
            auth: Authorization(Bearer { token: token.into() }),
            hyper_client: HyperClient::new()
        }
    }
    /// Creates a new HipChat API v2 client that has read and write timeouts
    pub fn with_timeouts<T: Into<String>, O: AsRef<str>>(origin: O, token: T, duration: Duration) -> Self {
        let mut hyper_client = HyperClient::new();
        hyper_client.set_read_timeout(Some(duration));
        hyper_client.set_write_timeout(Some(duration));

        Client {
            base_url: format!("{}/v2", origin.as_ref()),
            auth: Authorization(Bearer { token: token.into() }),
            hyper_client: hyper_client
        }
    }
    /// [Get emoticon](https://www.hipchat.com/docs/apiv2/method/get_emoticon)
    pub fn get_emoticon<T: AsRef<str>>(&self, emoticon_id_or_shortcut: T) -> Result<Emoticon, Error> {
        let mut res = try!(self.hyper_client.get(&format!("{}/emoticon/{}", self.base_url, emoticon_id_or_shortcut.as_ref()))
            .header(self.auth.to_owned())
            .send());

        if res.status.class() != StatusClass::Success {
            return Err(Error::HttpStatus(res.status));
        }

        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        Ok(try!(json::decode(&body)))
    }
    /// [Get room](https://www.hipchat.com/docs/apiv2/method/get_room)
    pub fn get_room<T: AsRef<str>>(&self, room_id_or_name: T) -> Result<RoomDetail, Error> {
        let mut res = try!(self.hyper_client.get(&format!("{}/room/{}", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .send());

        if res.status.class() != StatusClass::Success {
            return Err(Error::HttpStatus(res.status));
        }

        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        Ok(try!(json::decode(&body)))
    }
    /// [Update room](https://www.hipchat.com/docs/apiv2/method/update_room)
    pub fn update_room<T: AsRef<str>>(&self, room_id_or_name: T, req: &RoomUpdate) -> Result<RoomDetail, Error> {
        let body = json::encode(req).unwrap();
        let mut res = try!(self.hyper_client.put(&format!("{}/room/{}", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .header(ContentType::json())
            .body(body.as_bytes())
            .send());

        if res.status.class() != StatusClass::Success {
            return Err(Error::HttpStatus(res.status));
        }

        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        Ok(try!(json::decode(&body)))
    }
    /// [Delete room](https://www.hipchat.com/docs/apiv2/method/delete_room)
    pub fn delete_room<T: AsRef<str>>(&self, room_id_or_name: T) -> Result<(), Error> {
        let res = try!(self.hyper_client.delete(&format!("{}/room/{}", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .send());

        if res.status.class() != StatusClass::Success {
            return Err(Error::HttpStatus(res.status));
        }
        Ok(())
    }
    /// [Get all rooms](https://www.hipchat.com/docs/apiv2/method/get_all_rooms)
    pub fn get_rooms(&self, req: Option<&RoomsRequest>) -> Result<Rooms, Error> {
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

        let mut res = try!(self.hyper_client.get(url)
            .header(self.auth.to_owned())
            .send());

        if res.status.class() != StatusClass::Success {
            return Err(Error::HttpStatus(res.status));
        }

        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        Ok(try!(json::decode(&body)))
    }
    /// [Send message](https://www.hipchat.com/docs/apiv2/method/send_message)
    pub fn send_message<T: AsRef<str>, U: Into<String>>(&self, room_id_or_name: T, message: U) -> Result<RoomMessage, Error> {
        let mut obj = BTreeMap::new();
        obj.insert("message".to_owned(), Json::String(message.into()));
        let body = json::encode(&obj).unwrap();

        let mut res = try!(self.hyper_client.post(&format!("{}/room/{}/message", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .header(ContentType::json())
            .body(body.as_bytes())
            .send());

        if res.status.class() != StatusClass::Success {
            return Err(Error::HttpStatus(res.status));
        }

        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        Ok(try!(json::decode(&body)))
    }
    /// [Send room notification](https://www.hipchat.com/docs/apiv2/method/send_room_notification)
    pub fn send_notification<T: AsRef<str>>(&self, room_id_or_name: T, notification: &Notification) -> Result<(), Error> {
        let body = json::encode(notification).unwrap();
        let res = try!(self.hyper_client.post(&format!("{}/room/{}/notification", self.base_url, room_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .header(ContentType::json())
            .body(body.as_bytes())
            .send());

        if res.status.class() != StatusClass::Success {
            return Err(Error::HttpStatus(res.status));
        }
        Ok(())
    }
}
