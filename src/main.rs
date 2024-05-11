use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    usd: f64,
}

fn capitalize(token: &str) -> String {
    // Function to capitalize a given string reference
    // If string reference is null, returns a new string
    // Note that the first letter does not appear twice
    // Because of the behavior of next(), which advance the iterator
    // So the first char is exhausted
    let mut char_1 = token.chars();

    match char_1.next() {
        None => String::new(),
        Some(char) => char.to_uppercase().collect::<String>() + char_1.as_str(),
    }
}

fn collect_arg() -> String {
    // We assign arg to a match case, if no arg is provided, we exit with non 0 status code
    let arg = std::env::args()
        .nth(1)
        .expect("\n> Please enter token name, see https://www.coingecko.com/ for full list\n");

    arg
}

fn get_price(token: &str) -> f64 {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        token
    );
    let resp = reqwest::blocking::get(&url)
        .expect("Failed to fetch data from the API")
        .text()
        .expect("Failed to read response body");

    let price =
        serde_json::from_str::<HashMap<String, Price>>(&resp).expect("Failed to parse JSON");

    match price.get(token) {
        Some(data) => data.usd,
        None => panic!("\n> Invalid token name, see https://www.coingecko.com/ for full list\n"),
    }
}

fn main() {
    let token = collect_arg();

    let token_price = get_price(&token);
    println!("\n>> {}: {:.3} $\n", capitalize(&token), token_price)
}
