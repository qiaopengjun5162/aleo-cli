use anyhow::Result;
use aleo_rust_sdk::{AleoClient};
use snarkvm::console::program::ProgramID;
use snarkvm::prelude::{PrivateKey, TestnetV0, FromStr};

pub async fn run(node: &str, pk_opt: &Option<String>, to: &str, amount: u64, mode: &str) -> Result<()> {
    let pk_str = pk_opt.as_ref()
        .ok_or_else(|| anyhow::anyhow!("--private-key (or ALEO_PRIVATE_KEY env) required for transfers"))?;

    let private_key = PrivateKey::<TestnetV0>::from_str(pk_str)?;

    println!("=== Aleo Transfer ===");
    println!("🔑 From: (derived from private key)");
    println!("📬 To:   {to}");
    println!("💵 Amount: {amount} microcredits ({:.6} credits)", amount as f64 / 1_000_000.0);
    println!("🔀 Mode:  {mode}");

    match mode {
        "public" => transfer_public(node, &private_key, to, amount).await?,
        "private" => transfer_private(node, &private_key, to, amount).await?,
        _ => anyhow::bail!("Invalid mode '{}': use 'public' or 'private'", mode),
    }

    Ok(())
}

async fn transfer_public(
    node: &str,
    private_key: &PrivateKey<TestnetV0>,
    to: &str,
    amount: u64,
) -> Result<()> {
    let mut client = AleoClient::new(node)?;
    // Need pk_str as string for set_account
    client.set_account_from_private_key_str(&private_key.to_string())?;

    println!("\n📝 Step 1 — Dry-run: executing transfer_public locally...");
    let result = client.execute_local(
        "credits.aleo",
        "transfer_public",
        &[to.to_string(), format!("{amount}u64")],
    )?;
    println!("✅ Dry-run succeeded");
    println!("{:?}", result);

    println!("\n📡 Step 2 — Full pipeline: proving and broadcasting...");
    let program_id = ProgramID::from_str("credits.aleo")?;

    let tx_id = client.execute_and_broadcast(
        private_key,
        &program_id,
        "transfer_public",
        vec![to, &format!("{amount}u64")],
        100_000,  // base fee
        0,        // priority fee
    ).await?;

    println!("\n🎉 Transaction: {tx_id}");
    println!("🔗 https://testnet.explorer.provable.com/transaction/{tx_id}");

    println!("\n⏳ Waiting for confirmation...");
    client.network.wait_for_confirmation(&tx_id).await?;
    println!("✅ Transfer complete!");
    Ok(())
}

async fn transfer_private(
    node: &str,
    private_key: &PrivateKey<TestnetV0>,
    to: &str,
    amount: u64,
) -> Result<()> {
    let mut client = AleoClient::new(node)?;
    client.set_account_from_private_key_str(&private_key.to_string())?;

    // Step 1: Find unspent private credits records
    println!("\n📡 Step 1 — Scanning for unspent private records...");
    let records = client.find_private_credits_records().await?;

    if records.is_empty() {
        anyhow::bail!("No private credits.aleo records found. Fund your address first or use 'public' mode.");
    }

    println!("Found {} private record(s):", records.len());
    for (i, (_cipher, amt)) in records.iter().enumerate() {
        println!("  [{i}] {:.6} credits ({amt} microcredits)", *amt as f64 / 1_000_000.0);
    }

    // Find the first record with enough credits (records are sorted descending)
    let target_record = records.into_iter().find(|(_c, amt)| *amt >= amount)
        .ok_or_else(|| anyhow::anyhow!(
            "No single record has enough credits. Need {amount}, largest found record insufficient. \
             Use 'public' to consolidate or fund with smaller amounts."
        ))?;

    let (record_ciphertext, record_amount) = target_record;

    println!("\n✅ Selected record with {:.6} credits", record_amount as f64 / 1_000_000.0);
    println!("   Record ciphertext (first 80 chars): {}...", &record_ciphertext[..record_ciphertext.len().min(80)]);

    // Step 2: Dry-run transfer_private locally
    println!("\n📝 Step 2 — Dry-run: executing transfer_private locally...");
    let result = client.execute_local(
        "credits.aleo",
        "transfer_private",
        &[record_ciphertext.clone(), to.to_string(), format!("{amount}u64")],
    )?;
    println!("✅ Dry-run succeeded");
    println!("{:?}", result);

    // Step 3: Prove and broadcast
    println!("\n📡 Step 3 — Proving and broadcasting...");
    let program_id = ProgramID::from_str("credits.aleo")?;

    let tx_id = client.execute_and_broadcast(
        private_key,
        &program_id,
        "transfer_private",
        vec![&record_ciphertext, to, &format!("{amount}u64")],
        100_000,  // base fee
        0,        // priority fee
    ).await?;

    println!("\n🎉 Transaction: {tx_id}");
    println!("🔗 https://testnet.explorer.provable.com/transaction/{tx_id}");

    println!("\n⏳ Waiting for confirmation...");
    client.network.wait_for_confirmation(&tx_id).await?;
    println!("✅ Private transfer complete!");
    Ok(())
}
