use anyhow::Result;
use aleo_rust_sdk::{AleoClient, AleoHttpClient};

pub async fn run(node: &str, pk_opt: &Option<String>, address: Option<&str>) -> Result<()> {
    if let Some(addr) = address {
        // Query public balance for any address
        let client = AleoHttpClient::new(node)?;
        let val = client.fetch_mapping_value("credits.aleo", "account", addr).await?;
        match val {
            Some(s) => {
                let micro: u64 = s.trim().parse()?;
                println!("📊 Address: {addr}");
                println!("💰 Public balance: {} microcredits ({:.6} credits)", micro, micro as f64 / 1_000_000.0);
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

        // Public balance
        match client.get_balance().await? {
            Some(b) => println!("💰 Public balance: {} microcredits ({:.6} credits)", b, b as f64 / 1_000_000.0),
            None => println!("ℹ️  No public balance"),
        }

        // Private records using find_private_credits_records
        let records = client.find_private_credits_records().await?;
        let total_private = records.len();
        let private_total_micro: u64 = records.iter().map(|(_, amt)| amt).sum();
        if total_private > 0 {
            println!("🔒 Private records: {total_private} unspent record(s), {:.6} credits total",
                private_total_micro as f64 / 1_000_000.0);
            for (i, (_c, amt)) in records.iter().enumerate() {
                println!("   [{i}] {:.6} credits", *amt as f64 / 1_000_000.0);
            }
        } else {
            println!("🔒 Private records: none");
        }
    } else {
        anyhow::bail!("Provide either an --address or --private-key to query balance");
    }

    Ok(())
}
