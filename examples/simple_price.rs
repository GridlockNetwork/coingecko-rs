use coingecko::{Client, SimplePriceReq};

pub fn main() {
    let client = Client::new().unwrap();
    let req = SimplePriceReq::new("ethereum".into(), "eur,gbp,usd".into());
    println!("{:#?}", client.simple_price(req).unwrap().get("ethereum"));
}
