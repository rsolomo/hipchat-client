use std::str::FromStr;

use rustc_serialize::{Encodable, Decodable, Decoder, Encoder};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Privacy {
    Public,
    Private
}

impl Default for Privacy {
    fn default() -> Self {
        Privacy::Public
    }
}

impl FromStr for Privacy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(Privacy::Public),
            "private" => Ok(Privacy::Private),
            _ => Err(())
        }
    }
}

impl AsRef<str> for Privacy {
    fn as_ref(&self) -> &str {
        match *self {
            Privacy::Public => "public",
            Privacy::Private => "private"
        }
    }
}

impl Decodable for Privacy {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_str().and_then(|s| {
            s.parse().map_err(|_| d.error("invalid value for privacy"))
        })
    }
}

impl Encodable for Privacy {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(self.as_ref())
    }
}

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

    #[test]
    fn unit_serialize_privacy_private() {
        let actual = json::encode(&Privacy::Private).unwrap();
        let expected = "\"private\"";
        assert_eq!(actual, expected);
    }
}
