use rustc_serialize::{Decodable, Decoder};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct UsersRequest {
    pub start_index: Option<u64>,
    pub max_results: Option<u64>,
    pub include_guests: Option<bool>,
    pub include_deleted: Option<bool>
}

impl Default for UsersRequest {
    fn default() -> Self {
        UsersRequest {
            start_index: Some(0),
            max_results: Some(100),
            include_guests: Some(false),
            include_deleted: Some(false),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UsersLinks {
    pub self_: String,
    pub prev: Option<String>,
    pub next: Option<String>
}

impl Decodable for UsersLinks {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 3, |d| {
            Ok(UsersLinks {
                self_: try!(d.read_struct_field("self", 0, Decodable::decode)),
                prev: try!(d.read_struct_field("prev", 1, Decodable::decode)),
                next: try!(d.read_struct_field("next", 2, Decodable::decode))
            })
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Users {
    pub start_index: u64,
    pub max_results: u64,
    pub items: Vec<User>,
    pub links: UsersLinks
}

impl Decodable for Users {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 4, |d| {
            Ok(Users {
                start_index: try!(d.read_struct_field("startIndex", 0, Decodable::decode)),
                max_results: try!(d.read_struct_field("maxResults", 0, Decodable::decode)),
                items: try!(d.read_struct_field("items", 0, Decodable::decode)),
                links: try!(d.read_struct_field("links", 0, Decodable::decode)),
            })
        })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable)]
pub struct User {
    pub name: String,
    pub mention_name: String,
    pub id: u64,
    pub links: UserDetailLinks
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UserClient {
    pub version: Option<String>,
    pub client_type: Option<String>,
}

impl Decodable for UserClient {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 4, |d| {
            Ok(UserClient {
                version: try!(d.read_struct_field("version", 0, Decodable::decode)),
                client_type: try!(d.read_struct_field("type", 0, Decodable::decode)),
            })
        })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable)]
pub struct UserPresence {
    pub status: Option<String>,
    pub idle: Option<u64>,
    pub show: String,
    pub client: Option<UserClient>,
    pub is_online: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable)]
pub struct UserDetail {
    pub id: u64,
    pub xmpp_jid: Option<String>,
    pub name: String,
    pub mention_name: String,
    pub email: Option<String>,
    pub title: Option<String>,
    pub timezone: Option<String>,
    pub photo_url: Option<String>,
    pub presence: Option<UserPresence>,

    pub is_deleted: Option<bool>,
    pub is_guest: Option<bool>,
    pub is_group_admin: Option<bool>,

    pub created: Option<String>,
    pub last_active: Option<String>,
    pub links: UserDetailLinks,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UserDetailLinks {
    pub self_: String,
}

impl Decodable for UserDetailLinks {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 4, |d| {
            Ok(UserDetailLinks {
                self_: try!(d.read_struct_field("self", 0, Decodable::decode)),
            })
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, RustcDecodable)]
pub struct UserMessage {
    pub id: String,
    pub timestamp: String
}


#[cfg(test)]
mod test {
    use super::*;
    use rustc_serialize::json;

    #[test]
    fn unit_users_links() {
        let expected = UsersLinks {
            self_: "https://www.example.com".to_owned(),
            prev: Some("https://www.example.com".to_owned()),
            next: Some("https://www.example.com".to_owned())
        };
        let actual = json::decode::<UsersLinks>(r#"{
            "self":"https://www.example.com",
            "prev":"https://www.example.com",
            "next":"https://www.example.com"
        }"#).unwrap();
        assert_eq!(actual, expected);
    }
}
