use crate::Error;
use isahc::http::Request;
use isahc::HttpClient;
use serde::de::DeserializeOwned;
use std::io::Read;

pub fn get_json<T: DeserializeOwned>(client: &HttpClient, uri: &str) -> Result<T, Error> {
    let request = Request::get(uri)
        .header("content-type", "application/javascript")
        .body(())?;

    let mut bytes = Vec::new();

    client
        .send(request)?
        .into_body()
        .read_to_end(&mut bytes)?;


    serde_json::from_slice(&bytes).map_err(Error::from)
}
