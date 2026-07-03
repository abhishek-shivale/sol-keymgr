use clap::Parser;
use crate::commands::Commands;

pub mod commands;
fn main() {
    let cli_parser = commands::KeyMgr::parse();

    match &cli_parser.command {
        Commands::New => {
            println!("Cli commands hit new command!");
        },
        Commands::Pubkey => {
            println!("Cli commands hit pubkey command!");
        }
    }

}