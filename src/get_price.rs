use std::{fmt, collections::HashMap, error::Error};
use crate::utils;

#[derive(Debug)]
pub enum GetPriceError {
    FetchError(String),
    JsonDeserialize(String),
    
}

impl fmt::Display for GetPriceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPriceError::FetchError(ref err) => write!(f, "An error occured while fetching the API: {err}"),
            GetPriceError::JsonDeserialize(ref err) => write!(f, "An error occured while parsing the json: {err}"),
        }
    }
}

impl Error for GetPriceError {}

/// Function to get the price of a given token.
///
/// It fetches the price of a given token from the CoinGecko API.
pub async fn get_price(token: &str) -> Result<(), GetPriceError> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={token}&vs_currencies=usd",
    );

    let resp = reqwest::get(&url)
    .await.map_err(|e| GetPriceError::FetchError(e.to_string()))?
    .text().await
    .map_err(|e| GetPriceError::FetchError(e.to_string()))?;
    
    if resp.contains("error") {
        return Err(GetPriceError::FetchError(resp));
    }

    let price_object = serde_json::from_str::<HashMap<String, HashMap<String, f64>>>(&resp)
    .map_err(|e| GetPriceError::JsonDeserialize(e.to_string()))?;
    
    
    // Accessing price from object like: {"token": {"usd": XXX}}
    let price = match price_object.get(token) {
        Some(token) => match token.get("usd") {
            Some(value) => value,
            None => {
                return Err(GetPriceError::JsonDeserialize(format!("could not parse the following data: {resp}")));
            }
        },
        None => return Err(GetPriceError::JsonDeserialize(format!("could not parse the following data: {resp}")))
    };
    

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