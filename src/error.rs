use std::{error, fmt::{Display, Formatter, Result}};

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Hyper(hyper::http::Error),
    SerdeJson(serde_json::Error),
    MissingPokemon,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Error::Reqwest(e) => write!(f, "{}", e),
            Error::Hyper(e) => write!(f, "{}", e),
            Error::SerdeJson(e) => write!(f, "{}", e),
            Error::MissingPokemon => write!(f, "Description for requested Pokemon not found"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Reqwest(e) => Some(e),
            Error::Hyper(e) => Some(e),
            Error::SerdeJson(e) => Some(e),
            Error::MissingPokemon => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<hyper::http::Error> for Error {
    fn from(e: hyper::http::Error) -> Self {
        Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::SerdeJson(e)
    }
}
