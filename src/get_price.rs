use std::collections::HashMap;

use crate::utils;
use std::error::Error;

// #[derive(Debug)]
// struct FetchPriceError {
//     message: String,
// }

/// Function to get the price of a given token.
///
/// It fetches the price of a given token from the CoinGecko API.
pub async fn get_price(token: &str) -> Result<(), Box<dyn Error>> {
    let url = format!(
    "https://api.coingecko.com/api/v3/simple/price?ids={token}&vs_currencies=usd",
    );

    let resp = reqwest::get(&url).await?.text().await?;

    let price_object = serde_json::from_str::<HashMap<String, HashMap<String, f64>>>(&resp)?;

    let price = price_object.get(token).unwrap().get("usd").unwrap();

    let formatted_price: String;

    if *price > 1.0 {
        formatted_price = format!("{:.2}", price);
    } else {
        formatted_price = format!("{:.4}", price);
    }

    println!(
        "\n>> {}: {} $\n",
        utils::capitalize(&token),
        formatted_price
    );

    Ok(())

}