use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    usd: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Crypto {
    ethereum: Price,
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
    match std::env::args().nth(1) {
        Some(arg) => return capitalize(&arg),
        None => {
            panic!("\n> Please enter token name, see https://www.coingecko.com/ for full list\n");
        }
    };
}

fn get_price(token: &str) -> Result<f64, Box<dyn Error>> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        token
    );
    let resp = reqwest::blocking::get(&url)?.text()?;

    let price: Result<f64, Box<dyn Error>> = match serde_json::from_str::<Crypto>(&resp) {
        Ok(price) => Ok(price.ethereum.usd),
        Err(err) => panic!("Error: {err}"),
    };

    price
}
fn main() {
    let token = collect_arg();

    match get_price(&token) {
        Ok(price) => println!("\n>> {}: {} $\n", token, price),
        Err(_) => return,
    }
}
