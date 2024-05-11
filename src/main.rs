mod get_price;
mod utils;

fn main() {
    let known_tokens = vec![
        "ethereum",
        "bitcoin",
        "solana",
        "dogecoin",
        "avalanche-2",
        "polkadot",
        "chainlink",
    ]
    .iter()
    .map(|t| t.to_string())
    .collect();

    let tokens = utils::collect_cli_args();

    for token in tokens {
        get_price::get_price(&token, &known_tokens);
    }
}
