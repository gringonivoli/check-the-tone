use crate::cli::Cli;
use clap::Parser;

mod cli;
mod msg;

#[tokio::main]
async fn main() {
    if let Err(err) = Cli::parse().run().await {
        eprint!("{err}");
        std::process::exit(1);
    };
}
