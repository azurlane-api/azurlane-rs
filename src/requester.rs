//! Provides client implementation for the `reqwest` crate
//! 
//! # Examples
//!
//! Refer to the documentation for [`AzurLaneRequester`].
//!
//! [`AzurLaneRequester`]: trait.AzurLaneRequester.html

use std::io::Read;
use reqwest::{Client, RequestBuilder, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde_json;
use crate::{API_URL, Error, Result};
use crate::model::{ShipResponse, ShipsResponse, ConstructionResponse};

/// Used to specify what category you want to get the ships from when using `get_ships()`
pub enum Category {
    /// Rarity can be one of but isn't limited to `Super Rare`, `Normal`, `Rare`
    RARITY,
    /// Type can be one of but isn't limited to `Destroyer`, `Aircraft Carrier`, `Submarine`
    TYPE,
    /// Affiliation can be one of but isn't limited to `Ironblood`, `Sakura Empire`, `Eagle Union`
    AFFILIATION
}

impl Category {
    /// String representation of the enum value
    pub fn string(&self) -> &str {
        match self {
            Self::RARITY => "rarity",
            Self::TYPE => "type",
            Self::AFFILIATION => "affiliation"
        }
    }
}

/// Trait which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the implemenation for the `reqwest` Client, simply use the
/// trait:
///
/// ```rust,no_run
/// use azurlane::AzurLaneRequester;
/// ```
pub trait AzurLaneRequester {
    /// Get ship data by name.
    fn get_ship_by_name(&self, name: &str) -> Result<ShipResponse>;

    /// Get ship data by id.
    fn get_ship_by_id(&self, id: &str) -> Result<ShipResponse>;

    /// Get a list of ships from the order specified
    fn get_ships(&self, category: Category, value: &str) -> Result<ShipsResponse>;

    /// Get a list of ships matching the construction time
    fn get_build_info(&self, time: &str) -> Result<ConstructionResponse>;
}

impl AzurLaneRequester for Client {
    fn get_ship_by_name(&self, name: &str) -> Result<ShipResponse> {
        let uri = Url::parse(&format!("{}/ship?name={}", API_URL, name))?;
        
        handle_request::<ShipResponse>(self.get(uri))
    }

    fn get_ship_by_id(&self, id: &str) -> Result<ShipResponse> {
        let uri = Url::parse(&format!("{}/ship?id={}", API_URL, id))?;
        
        handle_request::<ShipResponse>(self.get(uri))
    }

    fn get_ships(&self, category: Category, value: &str) -> Result<ShipsResponse> {
        let uri = Url::parse(&format!("{}/ships?category={}&{}={}", API_URL, category.string(), category.string(), value))?;
        
        handle_request::<ShipsResponse>(self.get(uri))
    }

    fn get_build_info(&self, time: &str) -> Result<ConstructionResponse> {
        let uri = Url::parse(&format!("{}/build?time={}", API_URL, time))?;
        
        handle_request::<ConstructionResponse>(self.get(uri))
    }
}

fn handle_request<T: DeserializeOwned>(request: RequestBuilder) -> Result<T> {
    let response = request.send()?;

    match response.status() {
        StatusCode::OK => {},
        StatusCode::BAD_REQUEST => {
            return Err(Error::ReqwestBad(Box::new(response)));
        },
        StatusCode::UNAUTHORIZED => {
            return Err(Error::ReqwestUnauthorized(Box::new(response)));
        },
        _ => return Err(Error::ReqwestInvalid(Box::new(response))),
    }

    from_reader(response)
}

#[inline]
fn from_reader<T: DeserializeOwned, U: Read>(reader: U) -> Result<T> {
    serde_json::from_reader(reader).map_err(From::from)
}