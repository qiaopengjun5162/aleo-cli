use anyhow::Result;
use aleo_rust_sdk::AleoClient;
use snarkvm::console::program::ProgramID;
use snarkvm::prelude::{PrivateKey, TestnetV0, FromStr};

pub async fn run(
    node: &str,
    pk_opt: &Option<String>,
    program_id: &str,
    function_name: &str,
    inputs: &[String],
    base_fee: u64,
    priority_fee: u64,
) -> Result<()> {
    let pk_str = pk_opt
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("--private-key (or ALEO_PRIVATE_KEY env) required for execution"))?;
    let private_key = PrivateKey::<TestnetV0>::from_str(pk_str)?;

    println!("=== Aleo Execute ===\n");
    println!("📜 Program:    {program_id}");
    println!("🔧 Function:   {function_name}");
    println!("📥 Inputs:     {}", inputs.join(", "));
    println!("💵 Base fee:   {base_fee} microcredits");
    println!("⏫ Priority:   {priority_fee} microcredits");

    println!("\n📝 Step 1 — Dry-run: executing locally...");
    let mut client = AleoClient::new(node)?;
    client.set_account_from_private_key_str(pk_str)?;

    let local_inputs: Vec<String> = inputs.iter().map(|s| s.to_string()).collect();

    let result = client.execute_local(program_id, function_name, &local_inputs)?;
    println!("✅ Dry-run succeeded");
    println!("   Result: {result}");

    println!("\n📡 Step 2 — Proving and broadcasting...");
    let pid = ProgramID::from_str(program_id)?;

    let input_refs: Vec<&str> = inputs.iter().map(|s| s.as_str()).collect();

    let tx_id = client
        .execute_and_broadcast(&private_key, &pid, function_name, input_refs, base_fee, priority_fee)
        .await?;

    println!("\n🎉 Transaction: {tx_id}");
    println!("🔗 https://testnet.explorer.provable.com/transaction/{tx_id}");

    println!("\n⏳ Waiting for confirmation...");
    client.network.wait_for_confirmation(&tx_id).await?;
    println!("✅ Execution complete!");
    Ok(())
}
