#[macro_use]
extern crate derive_setters;
#[macro_use]
extern crate fomat_macros;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

mod coin_info;
mod simple_price;
mod utils;

use const_format::concatcp;

use isahc::HttpClient;

pub use crate::coin_info::*;
pub use crate::simple_price::*;

const API: &str = "https://api.coingecko.com/api/v3/";

pub struct Client {
    http: HttpClient,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        let http = isahc::HttpClient::new()?;
        Ok(Self { http })
    }

    /// Check if CoinGecko is reachable
    pub fn ping(&self) -> Result<Ping, Error> {
        const PING: &str = concatcp!(crate::API, "/ping");

        utils::get_json(&self.http, PING)
    }

    /// Fetches the current price of any cryptocurrencies in any other supported currencies you need.
    pub fn simple_price(&self, req: SimplePriceReq) -> Result<SimplePrices, Error> {
        const SIMPLE: &str = concatcp!(crate::API, "/simple/price");

        let uri = fomat!((SIMPLE) "?" (req.query()));

        utils::get_json(&self.http, &uri)
    }

    /// Fetches detailed information about a particular coin by its ID.
    pub fn coin_info(&self, coin: &str) -> Result<CoinInfo, Error> {
        const COINS: &str = concatcp!(crate::API, "/coins");

        let uri = fomat!((COINS) "/" (coin));

        utils::get_json(&self.http, &uri)
    }

    /// Fetches a list of coins supported by CoinGecko
    pub fn coins_list(&self) -> Result<Vec<Coin>, Error> {
        const COINS_LIST: &str = concatcp!(crate::API, "/coins/list");

        utils::get_json(&self.http, COINS_LIST)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error")]
    Http(#[from] isahc::Error),
    #[error("HTTP error")]
    HttpError(#[from] isahc::http::Error),
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Deserialization error")]
    Deserialization(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Ping {
    gecko_says: String,
}
