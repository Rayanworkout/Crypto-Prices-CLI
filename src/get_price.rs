use std::collections::HashMap;

use crate::utils;

/// Function to get the price of a given token.
///
/// It fetches the price of a given token from the CoinGecko API.
pub async fn get_price(token: &str) {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        token
    );
    let resp = reqwest::get(&url)
        .await
        .expect("Failed to fetch data from the API")
        .text()
        .await
        .expect("Failed to read response body");

    let price = serde_json::from_str::<HashMap<String, HashMap<String, f64>>>(&resp)
        .expect("Failed to parse JSON");

    match price.get(token) {
        Some(data) => {
            let price = data.get("usd").unwrap().to_owned();

            let formatted_price: String;

            if price > 1.0 {
                formatted_price = format!("{:.2}", price);
            } else {
                formatted_price = format!("{:.4}", price);
            }

            println!(
                "\n>> {}: {} $\n",
                utils::capitalize(&token),
                formatted_price
            )
        }
        None => {
            println!(
                "\n> Invalid token name \"{}\", see https://www.coingecko.com/ for full list.",
                &token
            );
        }
    }
}
