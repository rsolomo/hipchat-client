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

#[cfg(test)]
mod test {
    use super::*;
    use rustc_serialize::json;

    #[test]
    fn unit_serialize_privacy_private() {
        let actual = json::encode(&Privacy::Private).unwrap();
        let expected = "\"private\"";
        assert_eq!(actual, expected);
    }
}
