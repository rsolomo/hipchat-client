use std::str::FromStr;

use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

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

impl Serialize for Privacy {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.visit_str(self.as_ref())
    }
}

impl Deserialize for Privacy {
    fn deserialize<D: Deserializer>(d: &mut D) -> Result<Self, D::Error> {
        d.visit_str(PrivacyVisitor)
    }
}

struct PrivacyVisitor;

impl Visitor for PrivacyVisitor {
    type Value = Privacy;

    fn visit_str<E: Error>(&mut self, s: &str) -> Result<Self::Value, E> {
        match s.parse() {
            Ok(x) => Ok(x),
            Err(_) => Err(E::syntax("invalid value for privacy"))
        }
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

impl Serialize for Color {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.visit_str(self.as_ref())
    }
}

impl Deserialize for Color {
    fn deserialize<D: Deserializer>(d: &mut D) -> Result<Self, D::Error> {
        d.visit_str(ColorVisitor)
    }
}

struct ColorVisitor;

impl Visitor for ColorVisitor {
    type Value = Color;

    fn visit_str<E: Error>(&mut self, s: &str) -> Result<Self::Value, E> {
        match s.parse() {
            Ok(x) => Ok(x),
            Err(_) => Err(E::syntax("invalid value for color"))
        }
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

impl Serialize for MessageFormat {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.visit_str(self.as_ref())
    }
}

impl Deserialize for MessageFormat {
    fn deserialize<D: Deserializer>(d: &mut D) -> Result<Self, D::Error> {
        d.visit_str(MessageFormatVisitor)
    }
}

struct MessageFormatVisitor;

impl Visitor for MessageFormatVisitor {
    type Value = MessageFormat;

    fn visit_str<E: Error>(&mut self, s: &str) -> Result<Self::Value, E> {
        match s.parse() {
            Ok(x) => Ok(x),
            Err(_) => Err(E::syntax("invalid value for message_format"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn unit_deserialize_message_format_html() {
        let expected = MessageFormat::Html;
        let actual = serde_json::from_str::<MessageFormat>("\"html\"")
            .unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_deserialize_message_format_text() {
        let expected = MessageFormat::Text;
        let actual = serde_json::from_str::<MessageFormat>("\"text\"")
            .unwrap_or_else(|e| panic!("{}", e));
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_serialize_message_format_text() {
        let actual = serde_json::to_string(&MessageFormat::Text).unwrap();
        let expected = "\"text\"";
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_serialize_privacy_private() {
        let actual = serde_json::to_string(&Privacy::Private).unwrap();
        let expected = "\"private\"";
        assert_eq!(actual, expected);
    }
}
