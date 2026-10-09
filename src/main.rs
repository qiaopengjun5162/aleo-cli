use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
use commands::{balance, deploy, exec, generate, query, record, stablecoin, transfer, verify};

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
    /// Generate a new Aleo account (private key, view key, address)
    Generate,
    /// Deploy a .aleo program to the network
    Deploy {
        /// Path to the .aleo program source file
        program_path: String,
        /// Program name (without .aleo suffix)
        program_name: String,
        /// Optional priority fee in microcredits
        #[arg(long)]
        priority_fee: Option<u64>,
    },
    /// Execute a program function on-chain
    Exec {
        /// Program ID (e.g. credits.aleo)
        program_id: String,
        /// Function name (e.g. transfer_public)
        function_name: String,
        /// Input arguments
        inputs: Vec<String>,
        /// Base fee in microcredits
        #[arg(long, default_value = "100000")]
        base_fee: u64,
        /// Priority fee in microcredits
        #[arg(long, default_value = "0")]
        priority_fee: u64,
        /// Run locally only, no proof, no broadcast (equivalent to JS SDK run() without proveExecution)
        #[arg(long)]
        local: bool,
        /// Prove locally, no broadcast (equivalent to JS SDK run() with proveExecution=true)
        #[arg(long)]
        prove: bool,
    },
    /// Verify a transaction on chain (equivalent to JS SDK getTransaction / verifyExecution)
    Verify {
        /// Transaction ID (e.g. at1...)
        tx_id: String,
        /// Verify the ZK proof locally (equivalent to JS SDK verifyExecution)
        #[arg(long)]
        deep: bool,
    },
    /// Inspect private records owned by the current account
    Record {
        /// Optional program ID filter (e.g. credits.aleo)
        #[arg(long)]
        program: Option<String>,
        /// Include spent records in output
        #[arg(long)]
        include_spent: bool,
        /// Skip chain scan; just show cached records
        #[arg(long)]
        no_refresh: bool,
    },
    /// Query stablecoin freeze list Merkle tree (USAD/USDCx)
    Stablecoin {
        /// Stablecoin name: usad or usdcx
        stablecoin: String,
        /// Optional address to generate an exclusion proof for
        #[arg(long)]
        prove: Option<String>,
    },
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
        Commands::Generate => {
            generate::run()?;
        }
        Commands::Deploy {
            program_path,
            program_name,
            priority_fee,
        } => {
            deploy::run(
                &cli.node,
                &cli.private_key,
                program_path,
                program_name,
                *priority_fee,
            )
            .await?;
        }
        Commands::Exec {
            program_id,
            function_name,
            inputs,
            base_fee,
            priority_fee,
            local,
            prove,
        } => {
            if *local && *prove {
                anyhow::bail!("Cannot use --local and --prove together. Choose one mode.");
            }
            exec::run(
                &cli.node,
                &cli.private_key,
                program_id,
                function_name,
                inputs,
                *base_fee,
                *priority_fee,
                *local,
                *prove,
            )
            .await?;
        }
        Commands::Verify { tx_id, deep } => {
            verify::run(&cli.node, tx_id, *deep).await?;
        }
        Commands::Record {
            program,
            include_spent,
            no_refresh,
        } => {
            let pk = cli
                .private_key
                .ok_or_else(|| anyhow::anyhow!("--private-key required for record scanning"))?;
            record::run(
                &cli.node,
                &pk,
                program.as_deref(),
                *include_spent,
                *no_refresh,
            )
            .await?;
        }
        Commands::Stablecoin { stablecoin, prove } => {
            stablecoin::run(&cli.node, stablecoin, prove.as_deref()).await?;
        }
    }

    Ok(())
}
