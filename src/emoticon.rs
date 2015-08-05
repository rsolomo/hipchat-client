//use rustc_serialize::{Decodable, Decoder};

#[derive(Debug, Hash, Eq, PartialEq, RustcDecodable)]
pub struct Emoticon {
    pub width: u64,
    pub audio_path: Option<String>,
    pub id: u64,
    pub shortcut: String,
    pub height: String
}
