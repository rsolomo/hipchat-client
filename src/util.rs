use rustc_serialize::{Encodable, Decodable, Decoder, Encoder};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Privacy {
    Private,
    Public
}

impl Default for Privacy {
    fn default() -> Self {
        Privacy::Public
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

impl Encodable for Privacy {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(self.as_ref())
    }
}

impl Decodable for Privacy {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_str().and_then(|s| {
            match &s[..] {
                "public" => Ok(Privacy::Public),
                "private" => Ok(Privacy::Private),
                _ => Err(d.error(&format!("`{}` is not a valid privacy.", s)))
            }
        })
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

impl Encodable for Color {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(self.as_ref())
    }
}

impl Decodable for Color {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_str().and_then(|s| {
            match &s[..] {
                "yellow" => Ok(Color::Yellow),
                "green" => Ok(Color::Green),
                "red" => Ok(Color::Red),
                "purple" => Ok(Color::Purple),
                "gray" => Ok(Color::Gray),
                "random" => Ok(Color::Random),
                _ => Err(d.error(&format!("`{}` is not a valid color.", s)))
            }
        })
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

impl AsRef<str> for MessageFormat {
    fn as_ref(&self) -> &str {
        match *self {
            MessageFormat::Html => "html",
            MessageFormat::Text => "text"
        }
    }
}

impl Encodable for MessageFormat {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(self.as_ref())
    }
}

impl Decodable for MessageFormat {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_str().and_then(|s| {
            match &s[..] {
                "html" => Ok(MessageFormat::Html),
                "text" => Ok(MessageFormat::Text),
                _ => Err(d.error(&format!("`{}` is not a valid message_format.", s)))
            }
        })
    }
}
