use aleo_rust_sdk::AleoClient;
use anyhow::Result;
use snarkvm::console::program::ProgramID;
use snarkvm::prelude::{FromStr, PrivateKey};

#[allow(clippy::too_many_arguments)]
pub async fn run(
    node: &str,
    pk_opt: &Option<String>,
    program_id: &str,
    function_name: &str,
    inputs: &[String],
    base_fee: u64,
    priority_fee: u64,
    local_only: bool,
    prove_only: bool,
) -> Result<()> {
    let pk_str = pk_opt.as_ref().ok_or_else(|| {
        anyhow::anyhow!("--private-key (or ALEO_PRIVATE_KEY env) required for execution")
    })?;

    let mut client = AleoClient::new(node)?;
    client.set_account_from_private_key_str(pk_str)?;

    println!("=== Aleo Execute ===\n");
    println!("📜 Program:    {program_id}");
    println!("🔧 Function:   {function_name}");
    println!("📥 Inputs:     {}", inputs.join(", "));

    let local_inputs: Vec<String> = inputs.iter().map(|s| s.to_string()).collect();

    // ── Mode 1: --local (no proof, no broadcast) ──────────────────────
    if local_only {
        println!(
            "💡 Mode:       local (no proof — equivalent to JS SDK run() without proveExecution)"
        );
        // Load program into client for non-credits programs
        if program_id != "credits.aleo" {
            let program = client.network.fetch_program(program_id).await?;
            let program_str = program.to_string();
            let aleo_program = aleo_rust_sdk::AleoProgram::from_source(&program_str)?;
            client.set_program(aleo_program);
        }
        let result = client.execute_local(program_id, function_name, &local_inputs)?;
        println!("\n✅ Execution succeeded (no proof generated)");
        println!("   Output: {result}");
        return Ok(());
    }

    // ── Build inputs for prove/local_prove ────────────────────────────
    let pid = ProgramID::from_str(program_id)?;
    let input_refs: Vec<&str> = inputs.iter().map(|s| s.as_str()).collect();

    // ── Mode 2: --prove (prove locally, no broadcast) ────────────────
    if prove_only {
        println!(
            "💡 Mode:       local with proof (equivalent to JS SDK run() with proveExecution=true)"
        );
        println!("💵 Base fee:   {base_fee} microcredits");
        println!("⏫ Priority:   {priority_fee} microcredits");

        let tx_json = client
            .prove_execution(
                &PrivateKey::from_str(pk_str)?,
                &pid,
                function_name,
                input_refs,
                base_fee,
                priority_fee,
            )
            .await?;

        println!("\n✅ Proof generated (not broadcast)");
        println!("   Transaction JSON (first 500 chars):");
        println!("{}", &tx_json[..tx_json.len().min(500)]);
        return Ok(());
    }

    // ── Mode 3: default — prove + broadcast ──────────────────────────
    println!("💵 Base fee:   {base_fee} microcredits");
    println!("⏫ Priority:   {priority_fee} microcredits");

    // Dry-run for credits.aleo
    if program_id == "credits.aleo" {
        println!("\n📝 Dry-run: executing locally...");
        let result = client.execute_local(program_id, function_name, &local_inputs)?;
        println!("✅ Dry-run succeeded");
        println!("   Result: {result}");
    } else {
        println!("\nℹ️  Skipping dry-run (custom program — fetching from network)");
    }

    println!("\n📡 Proving and broadcasting...");
    let tx_id = client
        .execute_and_broadcast(
            &PrivateKey::from_str(pk_str)?,
            &pid,
            function_name,
            input_refs,
            base_fee,
            priority_fee,
        )
        .await?;

    println!("\n🎉 Transaction: {tx_id}");
    println!("🔗 https://testnet.explorer.provable.com/transaction/{tx_id}");

    println!("\n⏳ Waiting for confirmation...");
    client.network.wait_for_confirmation(&tx_id).await?;
    println!("✅ Execution complete!");
    Ok(())
}
