use aleo_rust_sdk::AleoHttpClient;
use anyhow::Result;

pub async fn run(node: &str) -> Result<()> {
    let client = AleoHttpClient::new(node)?;

    println!("=== Aleo Testnet Query ===\n");

    // Block height
    let height = client.fetch_block_height().await?;
    println!("📊 Block height:    {height}");

    // State root
    let state_root = client.fetch_state_root_only().await?;
    println!("🔬 State root:      {state_root}");

    // credits.aleo program
    match client.fetch_program("credits.aleo").await {
        Ok(p) => println!("📜 credits.aleo:   {} functions", p.functions().len()),
        Err(e) => println!("⚠️  credits.aleo:   {e}"),
    }

    // Known faucet address
    let faucet = "aleo1dev793afmhq2xwuv9k7uxrxxwljhyf9hysp883hs7p2ryq5z7pqsk2hp35";
    match client
        .fetch_mapping_value("credits.aleo", "account", faucet)
        .await
    {
        Ok(Some(b)) => println!("💰 Faucet balance:  {b} microcredits"),
        Ok(None) => println!("💰 Faucet balance:  none (private only)"),
        Err(e) => println!("⚠️  Faucet query:   {e}"),
    }

    println!("\n=== Done ===");
    Ok(())
}
