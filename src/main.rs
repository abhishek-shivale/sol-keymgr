use crate::commands::Commands;
use crate::commands::{activate, current, delete, env_cmd, export, import, import_key, init, kill, list, new, pubkey, recover, use_cmd};
use clap::Parser;

pub mod commands;
mod constant;
pub mod encryption;
pub mod error;
pub mod vault;

fn main() {
    let cli_parser = commands::KeyMgr::parse();

    let result: Result<(), Box<dyn std::error::Error>> = match cli_parser.command {
        Commands::Init { undo } => init::run(undo).map_err(Into::into),
        Commands::New { addr, no_toml } => new::run(addr, no_toml).map_err(Into::into),
        Commands::Import { file, addr } => import::run(file, addr).map_err(Into::into),
        Commands::ImportKey { addr } => import_key::run(addr).map_err(Into::into),
        Commands::Recover { addr } => recover::run(addr).map_err(Into::into),
        Commands::Export { addr, outfile } => export::run(addr, outfile).map_err(Into::into),
        Commands::Delete { addr } => delete::run(addr).map_err(Into::into),
        Commands::Activate { addr } => activate::run(addr).map_err(Into::into),
        Commands::Kill { addr, all } => kill::run(addr, all).map_err(Into::into),
        Commands::Use { addr, no_toml } => use_cmd::run(addr, no_toml).map_err(Into::into),
        Commands::Env { addr } => env_cmd::run(addr).map_err(Into::into),
        Commands::Current => current::run().map_err(Into::into),
        Commands::List { env, json } => list::run(env, json).map_err(Into::into),
        Commands::Pubkey { all } => pubkey::run(all).map_err(Into::into),
       };

    if let Err(err) = result {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
