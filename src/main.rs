
use clap::{Arg, Command};

use crate::market_data::fetch::fetch_market_data;

mod market_data;
mod trader;

fn cli() -> Command {
    Command::new("soltrader")
        .about("Places an order for a token on Raydium")
        .version("1.0")
        .author("GhostMac")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("check")
                .about("Checks the information about a token")
                .arg(
                    Arg::new("token-address")
                        .long("token-address")
                        .short('t')
                        .help("The public key of the token to check")
                        .required(true)
                )
        )
}

#[tokio::main]
async fn main() {
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

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("check", sub_matches)) => {
            // let token_address = Pubkey::from_str(sub_matches.get_one::<String>("token-address").unwrap()).unwrap();
            let token_address = sub_matches.get_one::<String>("token-address").unwrap();
            println!("Checking information for token: {}", token_address);
            let market_data = fetch_market_data(token_address).await.unwrap();
            println!("{:?}", market_data);
        }
        _ => unreachable!("The CLI should require a subcommand, this should never happen"),
    }
}
