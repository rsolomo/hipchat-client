use std::io::prelude::*;
use std::collections::BTreeMap;
use std::time::Duration;

use hyper::Client as HyperClient;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::Url;
use hyper::header::{Authorization, Bearer, ContentType};
use hyper::status::StatusClass;
use rustc_serialize::json::{self, Json};

use emoticon::Emoticon;
use error::Error;
use room::{RoomDetail, RoomUpdate, Rooms, RoomsRequest, Notification};
use user::{UserDetail, Users, UsersRequest};
use message::{Message, Messages, MessagesRequest};
use util::AppendToQueryParams;

const DEFAULT_TIMEOUT: u64 = 120;

pub struct Client {
    base_url: String,
    auth: Authorization<Bearer>,
    hyper_client: HyperClient
}

impl Client {
    /// Creates a new HipChat API v2 client
    pub fn new<T: Into<String>, O: AsRef<str>>(origin: O, token: T) -> Self {
        let duration = Duration::new(DEFAULT_TIMEOUT, 0);
        Client::with_timeouts(origin, token, duration)
    }
    /// Creates a new HipChat API v2 client that has read and write timeouts
    pub fn with_timeouts<T: Into<String>, O: AsRef<str>>(origin: O, token: T, duration: Duration) -> Self {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let mut hyper_client = HyperClient::with_connector(connector);
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
        let mut url = Url::parse(&format!("{}/room", self.base_url)).unwrap();
        req.map(|rooms_request| rooms_request.append_to(&mut url.query_pairs_mut()));

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
    pub fn send_message<T: AsRef<str>, U: Into<String>>(&self, room_id_or_name: T, message: U) -> Result<Message, Error> {
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
    /// [Get Private Messages](https://www.hipchat.com/docs/apiv2/method/view_privatechat_history)
    pub fn get_private_messages<T: AsRef<str>>(&self, user_id_or_email: T, req: Option<&MessagesRequest>) -> Result<Messages, Error> {
        let mut url = Url::parse(&format!("{}/user/{}/history", self.base_url, user_id_or_email.as_ref())).unwrap();
        req.map(|messages_request| messages_request.append_to(&mut url.query_pairs_mut()));

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
    /// [Get all users](https://www.hipchat.com/docs/apiv2/method/get_all_users)
    pub fn get_users(&self, req: Option<&UsersRequest>) -> Result<Users, Error> {
        let mut url = Url::parse(&format!("{}/user", self.base_url)).unwrap();
        req.map(|users_request| users_request.append_to(&mut url.query_pairs_mut()));

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
    /// [Get user](https://www.hipchat.com/docs/apiv2/method/view_user)
    pub fn get_user<T: AsRef<str>>(&self, user_id_or_name: T) -> Result<UserDetail, Error> {
        let mut res = try!(self.hyper_client.get(&format!("{}/user/{}", self.base_url, user_id_or_name.as_ref()))
            .header(self.auth.to_owned())
            .send());

        if res.status.class() != StatusClass::Success {
            return Err(Error::HttpStatus(res.status));
        }

        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        Ok(try!(json::decode(&body)))
    }
}
