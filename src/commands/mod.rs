pub mod activate;
pub mod current;
pub mod delete;
pub mod env_cmd;
pub mod export;
pub mod import;
pub mod import_key;
pub mod init;
pub mod kill;
pub mod list;
pub mod new;
pub mod pubkey;
pub mod recover;
pub mod use_cmd;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None, styles=get_styles())]
pub struct KeyMgr {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Set up the vault; convert ~/.config/solana/id.json into a keymgr-managed symlink")]
    Init {
        #[arg(long, help = "Undo init: restore the original id.json from backup")]
        undo: bool,
    },
    #[clap(about = "Generate a new keypair, encrypt it, and store it in the vault")]
    New {
        #[arg(help = "env/name for the new key, e.g. dev/deployer")]
        addr: Option<String>,
        #[arg(long, help = "Don't offer to bind this project to the new key")]
        no_toml: bool,
    },
    #[clap(about = "Import a solana-keygen JSON keypair file into the vault")]
    Import {
        #[arg(help = "Path to the solana-keygen JSON keypair file")]
        file: PathBuf,
        #[arg(help = "env/name to store it as, e.g. dev/deployer")]
        addr: Option<String>,
    },
    #[clap(about = "Import a base58 private key (e.g. exported from Phantom) into the vault")]
    ImportKey {
        #[arg(help = "env/name to store it as, e.g. dev/deployer")]
        addr: Option<String>,
    },
    #[clap(about = "Recover a keypair from a BIP39 seed phrase into the vault")]
    Recover {
        #[arg(help = "env/name to store it as, e.g. dev/deployer")]
        addr: Option<String>,
    },
    #[clap(about = "Decrypt a key and write it out as a solana-keygen JSON file")]
    Export {
        #[arg(help = "env/name of the key to export")]
        addr: String,
        #[arg(short, long, help = "Output path (default: {env}_{name}.json)")]
        outfile: Option<PathBuf>,
    },
    #[clap(about = "Remove a key from the vault (must not be active)")]
    Delete {
        #[arg(help = "env/name of the key to delete")]
        addr: String,
    },
    #[clap(about = "Decrypt a key into the active set")]
    Activate {
        #[arg(help = "env/name of the key to activate")]
        addr: String,
    },
    #[clap(about = "Wipe key(s) from the active set")]
    Kill {
        #[arg(help = "env/name of the key to kill (omit for interactive picker)")]
        addr: Option<String>,
        #[arg(long, help = "Wipe the entire active set")]
        all: bool,
    },
    #[clap(about = "Activate a key, point the default symlink at it, and bind this project")]
    Use {
        #[arg(help = "env/name to use (omit to read .keymgr.toml or pick interactively)")]
        addr: Option<String>,
        #[arg(long, help = "Don't create/update .keymgr.toml")]
        no_toml: bool,
    },
    #[clap(about = "Print export lines to consume a key in the current shell")]
    Env {
        #[arg(help = "env/name of an already-active key")]
        addr: String,
    },
    #[clap(about = "Show the default symlink target and the full active set")]
    Current,
    #[clap(about = "List keys in the vault")]
    List {
        #[arg(long, help = "Filter by env (local, dev, prod)")]
        env: Option<String>,
        #[arg(long, help = "Output as JSON")]
        json: bool,
    },
    #[clap(about = "Print the pubkey of the current project's key")]
    Pubkey {
        #[arg(long, help = "Pick from the whole vault instead of using .keymgr.toml")]
        all: bool,
    },
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}
