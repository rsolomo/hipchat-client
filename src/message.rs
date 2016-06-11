use std::str::FromStr;
use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};
use user::{UserDetail};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Color {
    Yellow,
    Green,
    Red,
    Purple,
    Gray,
    Random
}

impl Default for Color {
    fn default() -> Self {
        Color::Yellow
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yellow" => Ok(Color::Yellow),
            "green" => Ok(Color::Green),
            "red" => Ok(Color::Red),
            "purple" => Ok(Color::Purple),
            "gray" => Ok(Color::Gray),
            "random" => Ok(Color::Random),
            _ => Err(())
        }
    }
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        match *self {
            Color::Yellow => "yellow",
            Color::Green => "green",
            Color::Red => "red",
            Color::Purple => "purple",
            Color::Gray => "gray",
            Color::Random => "random"
        }
    }
}

impl Decodable for Color {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_str().and_then(|s| {
            s.parse().map_err(|_| d.error("invalid value for color"))
        })
    }
}

impl Encodable for Color {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(self.as_ref())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum MessageType {
    Message,
    GuestAccess,
    Topic,
    Notification
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Message
    }
}

impl FromStr for MessageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "message" => Ok(MessageType::Message),
            "guest_access" => Ok(MessageType::GuestAccess),
            "topic" => Ok(MessageType::Topic),
            "notification" => Ok(MessageType::Notification),
            _ => Err(())
        }
    }
}

impl AsRef<str> for MessageType {
    fn as_ref(&self) -> &str {
        match *self {
            MessageType::Message => "message",
            MessageType::GuestAccess => "guest_access",
            MessageType::Topic => "topic",
            MessageType::Notification => "notification"
        }
    }
}

impl Decodable for MessageType {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_str().and_then(|s| {
            s.parse().map_err(|_| d.error("invalid value for message type"))
        })
    }
}

impl Encodable for MessageType {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(self.as_ref())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum MessageFormat {
    Html,
    Text
}

impl Default for MessageFormat {
    fn default() -> Self {
        MessageFormat::Html
    }
}

impl FromStr for MessageFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "html" => Ok(MessageFormat::Html),
            "text" => Ok(MessageFormat::Text),
            _ => Err(())
        }
    }
}

impl AsRef<str> for MessageFormat {
    fn as_ref(&self) -> &str {
        match *self {
            MessageFormat::Html => "html",
            MessageFormat::Text => "text"
        }
    }
}

impl Decodable for MessageFormat {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_str().and_then(|s| {
            s.parse().map_err(|_| d.error("invalid value for message_format"))
        })
    }
}

impl Encodable for MessageFormat {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(self.as_ref())
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct MessagesRequest {
    pub start_index: Option<u64>,
    pub max_results: Option<u64>,
    pub reversed: Option<bool>,
    pub date: Option<String>,
    pub include_deleted: Option<bool>,
    pub timezone: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct MessageDetailLinks {
    pub self_: String,
}

impl Decodable for MessageDetailLinks {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 4, |d| {
            Ok(MessageDetailLinks {
                self_: try!(d.read_struct_field("self", 0, Decodable::decode)),
            })
        })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Messages {
    pub start_index: u64,
    pub max_results: u64,
    pub items: Vec<Message>,
    pub links: MessageDetailLinks
}

impl Decodable for Messages {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 4, |d| {
            Ok(Messages {
                start_index: try!(d.read_struct_field("startIndex", 0, Decodable::decode)),
                max_results: try!(d.read_struct_field("maxResults", 0, Decodable::decode)),
                items: try!(d.read_struct_field("items", 0, Decodable::decode)),
                links: try!(d.read_struct_field("links", 0, Decodable::decode)),
            })
        })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcDecodable)]
pub struct MessageFile {
    pub url: String,
    pub thumb_url: Option<String>,
    pub name: String,
    pub size: u64
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Message {
    pub id: String,
    pub date: String,
    pub from: Option<UserDetail>,
    pub message: String,
    pub message_format: Option<MessageFormat>,
    pub message_type: MessageType,
    pub color: Option<Color>,
    pub mentions: Vec<String>,
    pub file: Option<MessageFile>,
}

impl Decodable for Message {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("root", 4, |d| {
            Ok(Message {
                id: try!(d.read_struct_field("id", 0, Decodable::decode)),
                date: try!(d.read_struct_field("date", 0, Decodable::decode)),
                from: try!(d.read_struct_field("from", 0, Decodable::decode)),
                message: try!(d.read_struct_field("message", 0, Decodable::decode)),
                message_format: try!(d.read_struct_field("message_format", 0, Decodable::decode)),
                message_type: try!(d.read_struct_field("type", 0, Decodable::decode)),
                color: try!(d.read_struct_field("color", 0, Decodable::decode)),
                mentions: try!(d.read_struct_field("mentions", 0, Decodable::decode)),
                file: try!(d.read_struct_field("file", 0, Decodable::decode)),
            })
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rustc_serialize::json;

    #[test]
    fn unit_deserialize_message_format_html() {
        let expected = MessageFormat::Html;
        let actual = json::decode::<MessageFormat>("\"html\"")
            .unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_deserialize_message_format_text() {
        let expected = MessageFormat::Text;
        let actual = json::decode::<MessageFormat>("\"text\"")
            .unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_serialize_message_format_text() {
        let actual = json::encode(&MessageFormat::Text).unwrap();
        let expected = "\"text\"";
        assert_eq!(actual, expected);
    }
}
