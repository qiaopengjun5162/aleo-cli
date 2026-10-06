use anyhow::Result;
use aleo_rust_sdk::{AleoClient, AleoHttpClient};

/// Aleo-cli record command — inspect private records.
pub async fn run(
    node: &str,
    pk: &str,
    program_filter: Option<&str>,
    include_spent: bool,
    no_refresh: bool,
) -> Result<()> {
    let mut client = AleoClient::new(node)?;
    client.set_account_from_private_key_str(pk)?;
    let account = client.require_account()?;
    let http_client = AleoHttpClient::new(node)?;

    let mut mgr = aleo_rust_sdk::record::RecordManager::new(
        http_client,
        &account.view_key.to_string(),
    )?;

    if !no_refresh {
        println!("🔍 Scanning chain for private records (up to 100K blocks)...");
        mgr.scan().await?;
    }

    let all_records = mgr.records();
    let filtered: Vec<_> = all_records
        .iter()
        .filter(|r| {
            if !include_spent && r.spent {
                return false;
            }
            if let Some(p) = program_filter {
                r.program_id.as_str() == p
            } else {
                true
            }
        })
        .collect();

    let total: u64 = filtered
        .iter()
        .filter(|r| !r.spent && r.program_id == "credits.aleo")
        .map(|r| r.microcredits)
        .sum();

    println!(
        "🔑 Address:  {}",
        account.address_str()
    );
    println!("📦 Records:  {} found, {} shown", all_records.len(), filtered.len());
    println!("💰 Balance:  {:.6} credits ({total} microcredits)", total as f64 / 1_000_000.0);
    println!();

    if filtered.is_empty() {
        println!("(no matching records)");
        return Ok(());
    }

    for (i, rec) in filtered.iter().enumerate() {
        let spent = if rec.spent { " (spent)" } else { "" };
        println!(
            "─── [{i}] {:.6} credits ({}){spent} ───",
            rec.microcredits as f64 / 1_000_000.0,
            rec.program_id,
        );
        println!("     owner:     {}", rec.owner);
        println!("     ciphertext: {}…", &rec.ciphertext[..std::cmp::min(60, rec.ciphertext.len())]);
        if !rec.data.is_empty() {
            for (k, v) in &rec.data {
                println!("     {k}: {v}");
            }
        }
    }

    Ok(())
}
