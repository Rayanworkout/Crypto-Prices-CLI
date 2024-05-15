### A simple Rust CLI to query the current price of a cryptocurrency using the CoinGecko API.

A small command-line interface (CLI) project written in Rust. With it you can easily check the price of a cryptocurrency using the CoinGecko API through the command line.


## Deployment


Once the repo is cloned, you just have to run:

Linux / MacOS users:
```bash
chmod +x crypto_prices
./crypto_prices ethereum # or any other cryptocurrency
```

Windows users:
```bash
.\crypto_prices.exe ethereum # or any other cryptocurrency
```

If you wish to compile this small project on your local machine, follow these steps:

_Note: you need to have [Rust and cargo](https://doc.rust-lang.org/book/ch01-01-installation.html) installed. These steps will work under Linux, MacOS and Windows._

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