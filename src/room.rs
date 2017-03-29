use rustc_serialize::{Decodable, Decoder};

use util::{Privacy, AppendToQueryParams};
use message::{Color, MessageFormat};

use url::UrlQuery;
use url::form_urlencoded::Serializer;


#[derive(Debug, Hash, Eq, PartialEq)]
pub struct RoomsRequest {
    pub start_index: Option<u64>,
    pub max_results: Option<u64>,
    pub include_private: Option<bool>,
    pub include_archived: Option<bool>
}

impl AppendToQueryParams for RoomsRequest {
    fn append_to(&self, query: &mut Serializer<UrlQuery>){
        self.start_index.map(|start_index| query.append_pair("start-index", &start_index.to_string()));
        self.max_results.map(|max_results| query.append_pair("max-results", &max_results.to_string()));
        self.include_private.map(|include_private| query.append_pair("include-private", &include_private.to_string()));
        self.include_archived.map(|include_archived| query.append_pair("include-archived", &include_archived.to_string()));
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Rooms {
    pub start_index: u64,
    pub max_results: u64,
    pub items: Vec<Room>,
    pub links: RoomsLinks
}

impl Decodable for Rooms {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 4, |d| {
            Ok(Rooms {
                start_index: try!(d.read_struct_field("startIndex", 0, Decodable::decode)),
                max_results: try!(d.read_struct_field("maxResults", 0, Decodable::decode)),
                items: try!(d.read_struct_field("items", 0, Decodable::decode)),
                links: try!(d.read_struct_field("links", 0, Decodable::decode)),
            })
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct RoomsLinks {
    pub self_: String,
    pub prev: Option<String>,
    pub next: Option<String>
}

impl Decodable for RoomsLinks {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 3, |d| {
            Ok(RoomsLinks {
                self_: try!(d.read_struct_field("self", 0, Decodable::decode)),
                prev: try!(d.read_struct_field("prev", 1, Decodable::decode)),
                next: try!(d.read_struct_field("next", 2, Decodable::decode))
            })
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, RustcDecodable)]
pub struct Room {
    pub name: String,
    pub id: u64,
    pub links: RoomDetailLinks
}

#[derive(Debug, Hash, Eq, PartialEq, RustcDecodable)]
pub struct RoomDetail {
    pub xmpp_jid: String,
    pub statistics: RoomDetailStatistics,
    pub name: String,
    pub links: RoomDetailLinks,
    pub created: String,
    pub is_archived: bool,
    pub privacy: Privacy,
    pub is_guest_accessible: bool,
    pub topic: String,
    pub avatar_url: Option<String>,
    pub id: u64,
    pub guest_access_url: Option<String>
}

#[derive(Debug, Hash, Eq, PartialEq, RustcDecodable)]
pub struct RoomDetailStatistics {
    pub links: RoomDetailStatisticsLinks
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct RoomDetailStatisticsLinks {
    pub self_: String
}

impl Decodable for RoomDetailStatisticsLinks {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 1, |d| {
            Ok(RoomDetailStatisticsLinks {
                self_: try!(d.read_struct_field("self", 0, Decodable::decode))
            })
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct RoomDetailLinks {
    pub self_: String,
    pub webhooks: String,
    pub members: Option<String>,
    pub participants: String
}

impl Decodable for RoomDetailLinks {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 4, |d| {
            Ok(RoomDetailLinks {
                self_: try!(d.read_struct_field("self", 0, Decodable::decode)),
                webhooks: try!(d.read_struct_field("webhooks", 1, Decodable::decode)),
                members: try!(d.read_struct_field("members", 2, Decodable::decode)),
                participants: try!(d.read_struct_field("participants", 3, Decodable::decode))
            })
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, RustcDecodable)]
pub struct RoomDetailOwner {
    pub mention_name: String,
    pub id: u64,
    pub links: RoomDetailOwnerLinks,
    pub name: String
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct RoomDetailOwnerLinks {
    pub self_: String
}

impl Decodable for RoomDetailOwnerLinks {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 1, |d| {
            Ok(RoomDetailOwnerLinks {
                self_: try!(d.read_struct_field("self", 0, Decodable::decode))
            })
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct RoomUpdate {
    pub name: Option<String>,
    pub privacy: Option<Privacy>,
    pub is_archived: Option<bool>,
    pub is_guest_accessible: Option<bool>,
    pub topic: Option<String>,
    pub owner: Option<RoomUpdateOwner>
}

#[derive(Debug, Hash, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct RoomUpdateOwner {
    pub id: Option<String>
}

#[derive(Debug, Hash, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Notification {
    pub color: Color,
    pub message: String,
    pub notify: bool,
    pub message_format: MessageFormat
}

impl Default for Notification {
    fn default() -> Self {
        Notification {
            color: Color::default(),
            message: String::default(),
            notify: false,
            message_format: MessageFormat::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rustc_serialize::json;
    use url::Url;

    #[test]
    fn unit_rooms_links() {
        let expected = RoomsLinks {
            self_: "https://www.example.com".to_owned(),
            prev: Some("https://www.example.com".to_owned()),
            next: Some("https://www.example.com".to_owned())
        };
        let actual = json::decode::<RoomsLinks>(r#"{
            "self":"https://www.example.com",
            "prev":"https://www.example.com",
            "next":"https://www.example.com"
        }"#).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_default_rooms_request_should_create_empty_params(){
        let rooms_request = RoomsRequest{ start_index: None,
                                          max_results: None,
                                          include_private: None,
                                          include_archived: None };

        let mut url = Url::parse("https://rsolomo.github.io/hipchat-client/hipchat_client/index.html").unwrap();

        rooms_request.append_to(&mut url.query_pairs_mut());

        assert_eq!(Some(""), url.query());
    }

    #[test]
    fn unit_populated_rooms_request_should_create_encoded_params(){
        let rooms_request = RoomsRequest{ start_index: Some(1),
                                          max_results: Some(10),
                                          include_private: Some(true),
                                          include_archived: Some(true) };

        let mut url = Url::parse("https://rsolomo.github.io/hipchat-client/hipchat_client/index.html").unwrap();

        rooms_request.append_to(&mut url.query_pairs_mut());

        assert_eq!(Some("start-index=1&max-results=10&include-private=true&include-archived=true"), url.query());
    }
}
