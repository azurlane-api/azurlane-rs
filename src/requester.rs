use std::io::Read;
use reqwest::{Client, RequestBuilder, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde_json;
use crate::{API_URL, Error, Result};
use crate::model::{ShipResponse, ShipsResponse};

pub enum Order {
    RARITY,
    TYPE,
    AFFILIATION
}

impl Order {
    pub fn string(&self) -> &str {
        match self {
            Self::RARITY => "rarity",
            Self::TYPE => "type",
            Self::AFFILIATION => "affiliation"
        }
    }
}

pub trait AzurLaneRequester {
    fn get_ship_by_name(&self, name: &str) -> Result<ShipResponse>;

    fn get_ships(&self, order: Order, value: &str) -> Result<ShipsResponse>;
}

impl AzurLaneRequester for Client {
    fn get_ship_by_name(&self, name: &str) -> Result<ShipResponse> {
        let uri = Url::parse(&format!("{}/ship?name={}", API_URL, name))?;
        
        handle_request::<ShipResponse>(self.get(uri))
    }

    fn get_ships(&self, order: Order, value: &str) -> Result<ShipsResponse> {
        let uri = Url::parse(&format!("{}/ships?orderBy={}&{}={}", API_URL, order.string(), order.string(), value))?;
        
        handle_request::<ShipsResponse>(self.get(uri))
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