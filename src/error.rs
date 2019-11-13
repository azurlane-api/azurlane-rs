use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

use reqwest::{
    Error as ReqwestError,
    Response as ReqwestResponse,
    UrlError as ReqwestUrlError,
};

/// A result type to compose a successful value and the library's [`Error`]
/// type.
///
/// [`Error`]: enum.Error.html
pub type Result<T> = StdResult<T, Error>;

/// An error type to compose a singular error enum between various dependencies'
/// errors.
#[derive(Debug)]
pub enum Error {
    /// An error from the `serde_json` crate.
    ///
    /// A potential reason for this is when there is an error deserializing a
    /// JSON response body.
    Json(JsonError),
    /// An error from the `reqwest` crate when it is enabled.
    Reqwest(ReqwestError),
    /// An error indicating a bad request when using `reqwest`.
    ReqwestBad(Box<ReqwestResponse>),
    /// An error indicating an invalid request when using `reqwest`.
    ReqwestInvalid(Box<ReqwestResponse>),
    /// An error indicating a parsing issue when using `reqwest`.
    ReqwestParse(ReqwestUrlError),
    /// An error indicating an unathorized request when using `reqwest`.
    ReqwestUnauthorized(Box<ReqwestResponse>),
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::Reqwest(err)
    }
}

impl From<ReqwestUrlError> for Error {
    fn from(err: ReqwestUrlError) -> Self {
        Error::ReqwestParse(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Json(ref inner) => inner.description(),
            Error::Reqwest(ref inner) => inner.description(),
            Error::ReqwestBad(_) => "Request bad",
            Error::ReqwestInvalid(_) => "Request invalid",
            Error::ReqwestParse(ref inner) => inner.description(),
            Error::ReqwestUnauthorized(_) => "Request auth bad",
        }
    }
}