//! # azurlane
//! 
//! An azur lane api wrapper for the unofficial azur lane json api
//! 
//! ### Installation
//! 
//! Add the following to your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! azurlane = "1.0"
//! ```
//! 
//! ### Example
//! 
//! ```rust,no_run
//! use azurlane::{AzurLaneRequester, Order};
//! use reqwest::Client;
//! 
//! fn main() {
//!     let client = Client::new();
//!     
//!     let _ = match client.get_ships(Order::RARITY, "Super Rare") {
//!         Ok(response) => {
//!             for i in 0..response.ships.len() {
//!                 println!("[{}]: ({})", response.ships[i].id, response.ships[i].name)
//!             }
//!         }
//!         Err(why) => {
//!             panic!("{}", why)
//!         }
//!     };
//! ```
//! 
//! ### License
//! 
//! GPL-3.0, View the full license [here](https://github.com/azurlane-api/azurlane-rs/blob/master/LICENSE)
#![deny(missing_docs)]
#![crate_name = "azurlane"]
#![crate_type = "lib"]

extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod requester;
pub mod model;

mod error;

pub use error::{Error, Result};

pub use requester::{AzurLaneRequester, Order};

/// Base API Url
pub const API_URL: &'static str = "https://azurlane-api.appspot.com/v1";
