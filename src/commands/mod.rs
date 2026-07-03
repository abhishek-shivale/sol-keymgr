pub mod new;

use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct KeyMgr {
    #[command(subcommand)]
    pub command: Commands
}


#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Generate new keypair file from a random seed phrase and optional BIP39 passphrase")]
    New,
    #[clap(about = "Display the pubkey from a keypair file")]
    Pubkey
}