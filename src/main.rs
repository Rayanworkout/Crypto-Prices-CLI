use std::collections::HashMap;

mod utils;

fn get_price(token: &str) -> f64 {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        token
    );
    let resp = reqwest::blocking::get(&url)
        .expect("Failed to fetch data from the API")
        .text()
        .expect("Failed to read response body");

    let price = serde_json::from_str::<HashMap<String, HashMap<String, f64>>>(&resp)
        .expect("Failed to parse JSON");

    match price.get(token) {
        Some(data) => data.get("usd").unwrap().to_owned(),
        None => panic!("\n> Invalid token name, see https://www.coingecko.com/ for full list\n"),
    }
}

fn main() {
    let token = utils::collect_arg();

    let token_price = get_price(&token);
    println!("\n>> {}: {:.3} $\n", utils::capitalize(&token), token_price)
}
