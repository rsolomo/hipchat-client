use user::{UserDetail};

use util::AppendToQueryParams;
use url::UrlQuery;
use url::form_urlencoded::Serializer;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Message,
    #[serde(rename = "guest_access")]
    GuestAccess,
    Topic,
    Notification
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageFormat {
    Html,
    Text
}

impl Default for MessageFormat {
    fn default() -> Self {
        MessageFormat::Html
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct MessagesRequest {
    pub start_index: Option<u64>,
    pub max_results: Option<u64>,
    pub reversed: Option<bool>,
    pub date: Option<String>,
    pub include_deleted: Option<bool>,
    pub timezone: Option<String>,
    pub end_date: Option<String>,
}

impl AppendToQueryParams for MessagesRequest {
    fn append_to(&self, query: &mut Serializer<UrlQuery>){
        self.start_index.map(|start_index| query.append_pair("start-index", &start_index.to_string()));
        self.max_results.map(|max_results| query.append_pair("max-results", &max_results.to_string()));
        self.reversed.map(|reversed| query.append_pair("reversed", &reversed.to_string()));
        self.date.as_ref().map(|date| query.append_pair("date", date));
        self.include_deleted.map(|include_deleted| query.append_pair("include-deleted", &include_deleted.to_string()));
        self.timezone.as_ref().map(|timezone| query.append_pair("timezone", timezone));
        self.end_date.as_ref().map(|end_date| query.append_pair("end-date", end_date));
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
pub struct MessageDetailLinks {
    #[serde(rename = "self")]
    pub self_: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
pub struct Messages {
    #[serde(skip_deserializing)]
    pub start_index: u64,
    #[serde(skip_deserializing)]
    pub max_results: u64,
    pub items: Vec<Message>,
    pub links: MessageDetailLinks
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
pub struct MessageFile {
    pub url: String,
    pub thumb_url: Option<String>,
    pub name: String,
    pub size: u64
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendMessageResponse {
    pub id: String,
    pub timestamp: String
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
pub struct Message {
    pub id: String,
    pub date: String,
    pub from: Option<UserDetail>,
    pub message: String,
    pub message_format: Option<MessageFormat>,
    #[serde(rename(deserialize="type"))]
    pub message_type: MessageType,
    pub color: Option<Color>,
    pub mentions: Vec<String>,
    pub file: Option<MessageFile>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{self};
    use url::Url;

    #[test]
    fn unit_deserialize_message_format_html() {
        let expected = MessageFormat::Html;
        let actual: MessageFormat = serde_json::from_str("\"html\"")
            .unwrap_or_else(|e| panic!("{:?}", e));
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_deserialize_message_format_text() {
        let expected = MessageFormat::Text;
        let actual: MessageFormat = serde_json::from_str("\"text\"")
            .unwrap_or_else(|e| panic!("{:?}", e));
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_serialize_message_format_text() {
        let text = MessageFormat::Text;
        let actual = serde_json::to_string(&text).unwrap();
        let expected = "\"text\"";
        assert_eq!(actual, expected);
    }

    #[test]
    fn unit_default_messages_request_should_create_empty_params(){
        let messages_request = MessagesRequest{ start_index: None,
                                                max_results: None,
                                                reversed: None,
                                                date: None,
                                                include_deleted: None,
                                                timezone: None,
                                                end_date: None };

        let mut url = Url::parse("https://rsolomo.github.io/hipchat-client/hipchat_client/index.html").unwrap();

        messages_request.append_to(&mut url.query_pairs_mut());

        assert_eq!(Some(""), url.query());
    }

    #[test]
    fn unit_populated_messages_request_should_create_encoded_params(){
        let messages_request = MessagesRequest{ start_index: Some(1),
                                                max_results: Some(10),
                                                reversed: Some(false),
                                                date: Some("2017-03-20T12:00:00+4:00".to_string()),
                                                include_deleted: Some(false),
                                                timezone: Some("UTC".to_string()),
                                                end_date: Some("2017-03-20T13:00:00+4:00".to_string()) };

        let mut url = Url::parse("https://rsolomo.github.io/hipchat-client/hipchat_client/index.html").unwrap();

        messages_request.append_to(&mut url.query_pairs_mut());

        assert_eq!(Some("start-index=1&max-results=10&reversed=false&date=2017-03-20T12%3A00%3A00%2B4%3A00&include-deleted=false&timezone=UTC&end-date=2017-03-20T13%3A00%3A00%2B4%3A00"), url.query());
    }
}
