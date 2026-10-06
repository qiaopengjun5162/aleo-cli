use aleo_rust_sdk::record::RecordManager;
use aleo_rust_sdk::{AleoClient, AleoHttpClient};
use anyhow::Result;
use snarkvm::console::program::{
    Ciphertext as SnarkCiphertext, ProgramID, Record as SnarkRecord, Value,
};
use snarkvm::prelude::{FromStr, PrivateKey, TestnetV0, ViewKey};

pub async fn run(
    node: &str,
    pk_opt: &Option<String>,
    to: &str,
    amount: u64,
    mode: &str,
) -> Result<()> {
    let pk_str = pk_opt.as_ref().ok_or_else(|| {
        anyhow::anyhow!("--private-key (or ALEO_PRIVATE_KEY env) required for transfers")
    })?;

    let private_key = PrivateKey::<TestnetV0>::from_str(pk_str)?;

    println!("=== Aleo Transfer ===");
    println!("🔑 From: (derived from private key)");
    println!("📬 To:   {to}");
    println!(
        "💵 Amount: {amount} microcredits ({:.6} credits)",
        amount as f64 / 1_000_000.0
    );
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

    let tx_id = client
        .execute_and_broadcast(
            private_key,
            &program_id,
            "transfer_public",
            vec![to, &format!("{amount}u64")],
            100_000, // base fee
            0,       // priority fee
        )
        .await?;

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
    let account = client.require_account()?;

    // Step 1: Scan for unspent private records using RecordManager
    println!("\n📡 Step 1 — Scanning for unspent private records...");
    let http_client = AleoHttpClient::new(node)?;
    let view_key_str = account.view_key.to_string();
    let mut mgr = RecordManager::new(http_client, &view_key_str)?;
    let records = mgr.scan_recent(5_000).await?;

    let unspent: Vec<_> = records
        .iter()
        .filter(|r| r.program_id == "credits.aleo" && !r.spent)
        .collect();

    if unspent.is_empty() {
        anyhow::bail!(
            "No private credits.aleo records found. Fund your address first or use 'public' mode."
        );
    }

    println!("Found {} unspent record(s):", unspent.len());
    for (i, rec) in unspent.iter().enumerate() {
        println!(
            "  [{i}] {:.6} credits ({})",
            rec.microcredits as f64 / 1_000_000.0,
            rec.ciphertext.get(..40).unwrap_or("…")
        );
    }

    // Coin selection: find a single record with enough credits
    let selection = mgr.select_records(amount)?;
    let target = &selection.records[0]; // largest-first
    let record_ciphertext = target.ciphertext.clone();
    let record_amount = target.microcredits;

    println!(
        "\n✅ Selected record with {:.6} credits",
        record_amount as f64 / 1_000_000.0
    );
    println!(
        "   Record ciphertext (first 80 chars): {}...",
        &record_ciphertext[..record_ciphertext.len().min(80)]
    );

    println!("\n📡 Step 2 — Proving and broadcasting (dry-run skipped for private)...");
    let program_id = ProgramID::from_str("credits.aleo")?;

    // Decrypt the selected record for use as a Value input
    let view_key = ViewKey::try_from(private_key)?;
    let ciphertext_record =
        SnarkRecord::<TestnetV0, SnarkCiphertext<TestnetV0>>::from_str(&record_ciphertext)?;
    let plaintext_record = ciphertext_record.decrypt(&view_key)?;
    let record_value = Value::Record(plaintext_record);

    let tx_id = client
        .execute_and_broadcast_with_values(
            private_key,
            &program_id,
            "transfer_private",
            vec![
                record_value,
                Value::from_str(to)?,
                Value::from_str(&format!("{amount}u64"))?,
            ],
            100_000, // base fee
            0,       // priority fee
        )
        .await?;

    // Mark the record as spent
    mgr.mark_spent(&record_ciphertext);
    println!("   (updated local cache: record marked spent)");

    println!("\n🎉 Transaction: {tx_id}");
    println!("🔗 https://testnet.explorer.provable.com/transaction/{tx_id}");

    println!("\n⏳ Waiting for confirmation...");
    client.network.wait_for_confirmation(&tx_id).await?;
    println!("✅ Private transfer complete!");
    Ok(())
}
