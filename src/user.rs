use util::AppendToQueryParams;
use url::UrlQuery;
use url::form_urlencoded::Serializer;

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

impl AppendToQueryParams for UsersRequest {
    fn append_to(&self, query: &mut Serializer<UrlQuery>){
        self.start_index.map(|start_index| query.append_pair("start-index", &start_index.to_string()));
        self.max_results.map(|max_results| query.append_pair("max-results", &max_results.to_string()));
        self.include_guests.map(|include_guests| query.append_pair("include-guests", &include_guests.to_string()));
        self.include_deleted.map(|include_deleted| query.append_pair("include-deleted", &include_deleted.to_string()));
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
pub struct UsersLinks {
    #[serde(rename = "self")]
    pub self_: String,
    pub prev: Option<String>,
    pub next: Option<String>
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct Users {
    #[serde(rename = "startIndex")]
    pub start_index: u64,
    #[serde(rename = "maxResults")]
    pub max_results: u64,
    pub items: Vec<User>,
    pub links: UsersLinks
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
pub struct User {
    pub name: String,
    pub mention_name: String,
    pub id: u64,
    pub links: UserDetailLinks
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
pub struct UserClient {
    pub version: Option<String>,
    pub client_type: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
pub struct UserPresence {
    pub status: Option<String>,
    pub idle: Option<u64>,
    pub show: String,
    pub client: Option<UserClient>,
    pub is_online: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
pub struct UserDetailLinks {
    #[serde(rename = "self")]
    pub self_: String,
}

#[derive(Debug, Hash, Eq, PartialEq, Deserialize)]
pub struct UserMessage {
    pub id: String,
    pub timestamp: String
}


#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{self};
    use url::Url;

    #[test]
    fn unit_users_links() {
        let expected = UsersLinks {
            self_: "https://www.example.com".to_owned(),
            prev: Some("https://www.example.com".to_owned()),
            next: Some("https://www.example.com".to_owned())
        };
        let actual: UsersLinks = serde_json::from_str(r#"{
            "self":"https://www.example.com",
            "prev":"https://www.example.com",
            "next":"https://www.example.com"
        }"#).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_default_users_request_should_create_empty_params(){
        let users_request = UsersRequest{ start_index: None,
                                          max_results: None,
                                          include_guests: None,
                                          include_deleted: None };

        let mut url = Url::parse("https://rsolomo.github.io/hipchat-client/hipchat_client/index.html").unwrap();

        users_request.append_to(&mut url.query_pairs_mut());

        assert_eq!(Some(""), url.query());
    }

    #[test]
    fn unit_populated_users_request_should_create_encoded_params(){
        let users_request = UsersRequest{ start_index: Some(1),
                                          max_results: Some(10),
                                          include_guests: Some(true),
                                          include_deleted: Some(true) };

        let mut url = Url::parse("https://rsolomo.github.io/hipchat-client/hipchat_client/index.html").unwrap();

        users_request.append_to(&mut url.query_pairs_mut());

        assert_eq!(Some("start-index=1&max-results=10&include-guests=true&include-deleted=true"), url.query());
    }
}
