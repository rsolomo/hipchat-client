use url::UrlQuery;
use url::form_urlencoded::Serializer;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Privacy {
    Public,
    Private
}

impl Default for Privacy {
    fn default() -> Self {
        Privacy::Public
    }
}

pub trait AppendToQueryParams {
    fn append_to(&self, query: &mut Serializer<UrlQuery>);
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{self};

    #[test]
    fn unit_serialize_privacy_private() {
        let actual = serde_json::to_string(&Privacy::Private).unwrap();
        let expected = "\"private\"";
        assert_eq!(actual, expected);
    }
}
