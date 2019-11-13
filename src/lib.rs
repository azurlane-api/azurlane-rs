extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod requester;
pub mod model;

mod error;

pub use error::{Error, Result};

pub use requester::AzurLaneRequester;

pub const API_URL: &'static str = "https://azurlane-api.appspot.com/v1";