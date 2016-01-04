#[derive(Debug, Clone, Hash, Eq, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Emoticon {
    pub width: u64,
    pub audio_path: Option<String>,
    pub id: u64,
    pub shortcut: String,
    pub height: String
}
