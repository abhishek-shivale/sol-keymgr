pub mod new;
pub mod pubkey;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct KeyMgr {
    #[command(subcommand)]
    pub command: Commands
}


#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Generate new keypair file from a random seed phrase and optional BIP39 passphrase")]
    New {
        #[arg(help = "Output path for the new keypair file")]
        outfile: Option<PathBuf>,
    },
    #[clap(about = "Display the pubkey from a keypair file")]
    Pubkey {
        #[arg(help = "Path to the keypair file")]
        keypair: Option<PathBuf>,
    }
}