pub fn main() {
    smol::block_on(async {

        let client = coingecko::Client::new();

        println!("{:#?}", client.coin_info("algorand"));
    })
}
