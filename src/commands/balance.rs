use aleo_rust_sdk::record::RecordManager;
use aleo_rust_sdk::{AleoClient, AleoHttpClient};
use anyhow::Result;

pub async fn run(node: &str, pk_opt: &Option<String>, address: Option<&str>) -> Result<()> {
    if let Some(addr) = address {
        // Public balance only (cannot query private without view key)
        let client = AleoHttpClient::new(node)?;
        let val = client
            .fetch_mapping_value("credits.aleo", "account", addr)
            .await?;
        match val {
            Some(s) => {
                let micro: u64 = s.trim().parse()?;
                println!("📊 Address: {addr}");
                println!(
                    "💰 Public balance: {micro} microcredits ({:.6} credits)",
                    micro as f64 / 1_000_000.0
                );
            }
            None => {
                println!("📊 Address: {addr}");
                println!("ℹ️  No public balance (address may only have private records)");
            }
        }
    } else if let Some(pk) = pk_opt {
        let mut client = AleoClient::new(node)?;
        client.set_account_from_private_key_str(pk)?;
        let account = client.require_account()?;
        println!("🔑 Address: {}", account.address_str());

        // ── Public balance ────────────────────────────────────────────
        match client.get_balance().await? {
            Some(b) => println!(
                "💰 Public balance: {b} microcredits ({:.6} credits)",
                b as f64 / 1_000_000.0
            ),
            None => println!("ℹ️  No public balance"),
        }

        // ── Private balance via RecordManager ────────────────────────
        let http_client = AleoHttpClient::new(node)?;
        let view_key_str = account.view_key.to_string();
        let mut mgr = RecordManager::new(http_client, &view_key_str)?;

        println!("🔍 Scanning blocks for private records...");
        let records = mgr.scan().await?;
        let private_total: u64 = records
            .iter()
            .filter(|r| r.program_id == "credits.aleo" && !r.spent)
            .map(|r| r.microcredits)
            .sum();

        if records.is_empty() {
            println!("🔒 Private records: none");
        } else {
            println!(
                "🔒 Private records: {} record(s), {:.6} credits total",
                records.len(),
                private_total as f64 / 1_000_000.0
            );
            for (i, rec) in records.iter().enumerate() {
                let owner_tag = if rec.owner == account.address {
                    "self"
                } else {
                    "other"
                };
                println!(
                    "   [{i}] {:.6} credits ({owner_tag}) {}",
                    rec.microcredits as f64 / 1_000_000.0,
                    rec.program_id
                );
            }
        }
    } else {
        anyhow::bail!("Provide either an --address or --private-key to query balance");
    }

    Ok(())
}
