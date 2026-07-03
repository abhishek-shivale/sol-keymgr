use clap::Parser;
use crate::commands::Commands;
use crate::commands::{new, pubkey};
use crate::constant::DEFAULT_PATH;
use std::path::PathBuf;

pub mod commands;
mod constant;

fn main() {
    let cli_parser = commands::KeyMgr::parse();

    let result = match cli_parser.command {
        Commands::New { outfile } => {
            let outfile = outfile.unwrap_or_else(|| {
                let default_path = PathBuf::from(DEFAULT_PATH);
                println!("using default output path: {:?}", default_path);
                default_path
            });
            new::run(outfile)
        },
        Commands::Pubkey { keypair } => {
            let keypair = keypair.unwrap_or_else(|| {
                let default_path = PathBuf::from(DEFAULT_PATH);
                println!("using default keypair path: {:?}", default_path);
                default_path
            });
            pubkey::run(keypair)
        },
    };

    if let Err(err) = result {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}