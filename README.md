### A simple Rust CLI to query the current price of a cryptocurrency using the CoinGecko API.

A small command-line interface (CLI) project written in Rust. With it you can easily check the price of a cryptocurrency using the CoinGecko API through the command line.


## Deployment

If you wish to install this small project on your local machine, follow these steps:

_Note: you need to have Rust and cargo installed on your machine. These steps will work under Linux, MacOS and Windows._

If you don't want to build the project yourself, you can download the binary from [here](/crypto_prices)

```bash
  git clone https://github.com/Rayanworkout/Crypto-Prices-CLI.git
```
```bash
  cd Crypto-Prices-CLI
  cargo build --release
```

Linux / MacOS users:
```bash
./target/release/crypto_prices ethereum # or any other cryptocurrency
```

Windows users:


```bash
.\target\release\crypto_prices.exe ethereum # or any other cryptocurrency
```

You can now easily setup an alias to run the program from anywhere in your terminal.