# SolTrader

```rust
println!(
        r#"

  ██████  ▒█████   ██▓  ▄▄▄█████▓ ██▀███   ▄▄▄      ▓█████▄ ▓█████  ██▀███
▒██    ▒ ▒██▒  ██▒▓██▒  ▓  ██▒ ▓▒▓██ ▒ ██▒▒████▄    ▒██▀ ██▌▓█   ▀ ▓██ ▒ ██▒
░ ▓██▄   ▒██░  ██▒▒██░  ▒ ▓██░ ▒░▓██ ░▄█ ▒▒██  ▀█▄  ░██   █▌▒███   ▓██ ░▄█ ▒
  ▒   ██▒▒██   ██░▒██░  ░ ▓██▓ ░ ▒██▀▀█▄  ░██▄▄▄▄██ ░▓█▄   ▌▒▓█  ▄ ▒██▀▀█▄
▒██████▒▒░ ████▓▒░░██████▒▒██▒ ░ ░██▓ ▒██▒ ▓█   ▓██▒░▒████▓ ░▒████▒░██▓ ▒██▒
▒ ▒▓▒ ▒ ░░ ▒░▒░▒░ ░ ▒░▓  ░▒ ░░   ░ ▒▓ ░▒▓░ ▒▒   ▓▒█░ ▒▒▓  ▒ ░░ ▒░ ░░ ▒▓ ░▒▓░
░ ░▒  ░ ░  ░ ▒ ▒░ ░ ░ ▒  ░  ░      ░▒ ░ ▒░  ▒   ▒▒ ░ ░ ▒  ▒  ░ ░  ░  ░▒ ░ ▒░
░  ░  ░  ░ ░ ░ ▒    ░ ░   ░        ░░   ░   ░   ▒    ░ ░  ░    ░     ░░   ░
      ░      ░ ░      ░  ░          ░           ░  ░   ░       ░  ░   ░
                                                     ░

        "#
    );
```

## About

SolTrader is my trading bot for Solana tokens. I am researching advanced order execution types like:

- Immediate-Or-Cancel (IOC) Orders / Fill-Or-Kill (FOK) Orders: to ensure that any order that cannot be filled immediately is cancelled/killed.
- Post-Only Orders: to ensure my order adds liquidity to the order book instead of taking it.

## Installation and Usage

Run:
```shell
git clone https://github.com/theghostmac/solana-trading-bot.git

cd solana-trading-bot

cargo build

./target/debug/solana-trading-bot --help
```

I plan to add a config to be updated with `.env` later.