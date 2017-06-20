extern crate hyper;
extern crate hyper_native_tls;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate url;

pub mod client;
pub mod error;
pub mod emoticon;
pub mod room;
pub mod user;
pub mod message;
pub mod util;

pub use client::Client;
