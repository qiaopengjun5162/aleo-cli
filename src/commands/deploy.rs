use anyhow::Result;
use aleo_rust_sdk::AleoClient;
use std::fs;

pub async fn run(
    node: &str,
    pk_opt: &Option<String>,
    program_path: &str,
    program_name: &str,
    priority_fee: Option<u64>,
) -> Result<()> {
    let _pk_str = pk_opt
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("--private-key (or ALEO_PRIVATE_KEY env) required for deploy"))?;

    println!("=== Aleo Deploy ===\n");

    // Read program source from file
    println!("📂 Reading program from: {program_path}");
    let source = fs::read_to_string(program_path)?;
    println!("📜 Program source ({} bytes)", source.len());
    println!();
    for line in source.lines().take(10) {
        println!("  {line}");
    }
    if source.lines().count() > 10 {
        println!("  ... ({} more lines)", source.lines().count() - 10);
    }

    println!("\n🔄 Deploying program '{program_name}.aleo'...");
    println!("⏳ This will take 30–60s (Varuna proving)...");

    let mut client = AleoClient::new(node)?;
    client.set_account_from_private_key_str(_pk_str)?;

    let pf = priority_fee.unwrap_or(0);
    let tx_id = client.deploy_program(&source, pf).await?;

    println!("\n✅ Deploy successful!");
    println!("   Transaction: {tx_id}");
    println!("   🔗 https://testnet.explorer.provable.com/transaction/{tx_id}");

    Ok(())
}
