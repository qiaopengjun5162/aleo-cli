use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
use commands::{balance, query, transfer};

/// Aleo CLI — interact with the Aleo blockchain from your terminal.
#[derive(Parser)]
#[command(name = "aleo")]
#[command(about = "Aleo blockchain CLI tool", long_about = None)]
struct Cli {
    /// Provable v2 REST API base URL
    #[arg(long, default_value = "https://api.explorer.provable.com/v2/testnet")]
    node: String,

    /// Private key for the sender account (env: ALEO_PRIVATE_KEY)
    #[arg(long, env = "ALEO_PRIVATE_KEY")]
    private_key: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check public balance of an address (default: current account)
    Balance {
        /// Optional address to check (otherwise uses the account's address)
        address: Option<String>,
    },
    /// Transfer credits (public or private)
    Transfer {
        /// Recipient address
        to: String,
        /// Amount in microcredits (e.g. 1000000 = 0.001 credits)
        amount: u64,
        /// Transfer mode: public (default) or private
        #[arg(long, default_value = "public")]
        mode: String,
    },
    /// Query Aleo testnet state
    Query,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Balance { address } => {
            balance::run(&cli.node, &cli.private_key, address.as_deref()).await?;
        }
        Commands::Transfer { to, amount, mode } => {
            transfer::run(&cli.node, &cli.private_key, to, *amount, mode).await?;
        }
        Commands::Query => {
            query::run(&cli.node).await?;
        }
    }

    Ok(())
}
