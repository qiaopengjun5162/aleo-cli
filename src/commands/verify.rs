use anyhow::Result;
use aleo_rust_sdk::AleoClient;
use serde_json::Value;

pub async fn run(
    node: &str,
    tx_id: &str,
    deep: bool,
) -> Result<()> {
    let client = AleoClient::new(node)?;

    println!("=== Aleo Verify ===\n");
    println!("🔍 Transaction: {tx_id}");

    // Show basic info first
    let raw = client.network.fetch_transaction(tx_id).await?;
    let v: Value = serde_json::from_str(&raw)?;
    let tx_type = v.get("type").and_then(|t| t.as_str()).unwrap_or("unknown");
    let status = v.get("status").and_then(|s| s.as_str()).unwrap_or("confirmed");

    println!("📋 Type:   {tx_type}");
    println!("📋 Status: {status}");

    if let Some(owner) = v.get("owner").and_then(|o| o.get("address")).and_then(|a| a.as_str()) {
        println!("👤 Owner:  {}", &owner[..20.min(owner.len())]);
    }

    // Print execution info
    if let Some(execution) = v.get("execution") {
        if let Some(transitions) = execution.get("transitions").and_then(|t| t.as_array()) {
            for (i, t) in transitions.iter().enumerate() {
                let pid = t.get("program_id").and_then(|p| p.as_str()).unwrap_or("?");
                let fn_name = t.get("function_name").and_then(|f| f.as_str()).unwrap_or("?");
                println!("\n🔧 Transition #{i}: {pid}::{fn_name}");
                if let Some(inputs) = t.get("inputs").and_then(|i| i.as_array()) {
                    for inp in inputs {
                        if let Some(val) = inp.get("value").and_then(|v| v.as_str()) {
                            println!("   Input: {val}");
                        }
                    }
                }
            }
        }
    }

    // Print deployment info
    if let Some(deployment) = v.get("deployment") {
        if let Some(pid) = deployment.get("program_id").and_then(|p| p.as_str()) {
            println!("\n📦 Deployed program: {pid}");
        }
    }

    if deep {
        println!("\n🔬 Deep verification (proof) enabled...");
        let result = client.verify_execution(tx_id).await?;
        println!("\n{result}");
    } else {
        println!("\n✅ Transaction exists on chain");
        println!("   Use --deep to also verify the ZK proof locally");
    }

    Ok(())
}
