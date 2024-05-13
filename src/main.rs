mod get_price;
mod utils;

#[tokio::main]
async fn main() {
    let known_tokens = [
        "ethereum",
        "bitcoin",
        "solana",
        "dogecoin",
        "avalanche-2",
        "polkadot",
        "chainlink",
    ]
    .map(|t| t.to_string());

    let tokens = utils::collect_cli_args();

    for token in tokens {
        if !known_tokens.contains(&token.to_lowercase()) {
            println!(
                "\n> \"{}\" is not in the tokens list, do you want to make the API call anyway ? y/n",
                token
            );

            match utils::confirm_api_call() {
                true => get_price::get_price(&token).await,
                false => println!("Aborting API call for \"{}\"", &token),
            }
        } else {
            get_price::get_price(&token).await
        }
    }
}
