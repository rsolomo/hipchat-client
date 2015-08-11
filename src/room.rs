use util::{Color, MessageFormat, Privacy};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct RoomsRequest {
    pub start_index: Option<u64>,
    pub max_results: Option<u64>,
    pub include_private: Option<bool>,
    pub include_archived: Option<bool>
}

#[allow(non_snake_case)]
#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct Rooms {
    pub startIndex: u64,
    pub maxResults: u64,
    pub items: Vec<Room>,
    pub links: RoomsLinks
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct RoomsLinks {
    #[serde(rename="self")]
    pub self_: String,
    pub prev: Option<String>,
    pub next: Option<String>
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct Room {
    pub name: String,
    pub id: u64,
    pub links: RoomDetailLinks
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
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

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct RoomDetailStatistics {
    pub links: RoomDetailStatisticsLinks
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct RoomDetailStatisticsLinks {
    #[serde(rename="self")]
    pub self_: String
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct RoomDetailLinks {
    #[serde(rename="self")]
    pub self_: String,
    pub webhooks: String,
    pub members: Option<String>,
    pub participants: String
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct RoomDetailOwner {
    pub mention_name: String,
    pub id: u64,
    pub links: RoomDetailOwnerLinks,
    pub name: String
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct RoomDetailOwnerLinks {
    #[serde(rename="self")]
    pub self_: String
}

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoomUpdate {
    pub name: Option<String>,
    pub privacy: Option<Privacy>,
    pub is_archived: Option<bool>,
    pub is_guest_accessible: Option<bool>,
    pub topic: Option<String>,
    pub owner: Option<RoomUpdateOwner>
}

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoomUpdateOwner {
    pub id: Option<String>
}

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoomMessage {
    pub id: String,
    pub timestamp: String
}

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
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
    use serde_json;

    #[test]
    fn unit_rooms_links() {
        let expected = RoomsLinks {
            self_: "https://www.example.com".to_owned(),
            prev: Some("https://www.example.com".to_owned()),
            next: Some("https://www.example.com".to_owned())
        };
        let actual = serde_json::from_str::<RoomsLinks>(r#"{
            "self":"https://www.example.com",
            "prev":"https://www.example.com",
            "next":"https://www.example.com"
        }"#).unwrap();
        assert_eq!(actual, expected);
    }
}
