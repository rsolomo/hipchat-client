#![feature(custom_derive, plugin, duration, custom_attribute)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod client;
pub mod error;
pub mod emoticon;
pub mod room;
pub mod util;

pub use client::Client;
