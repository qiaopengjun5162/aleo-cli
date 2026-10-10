use aleo_rust_sdk::AleoClient;
use anyhow::Result;

/// Parse a raw transaction JSON response and extract key fields for display.
/// Pure function — no network calls, so testable without a node.
pub fn parse_transaction_summary(raw: &str) -> Result<TransactionSummary> {
    let v: serde_json::Value = serde_json::from_str(raw)?;
    let tx_type = v
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("unknown")
        .to_string();
    let status = v
        .get("status")
        .and_then(|s| s.as_str())
        .unwrap_or("confirmed")
        .to_string();
    let owner = v
        .get("owner")
        .and_then(|o| o.get("address"))
        .and_then(|a| a.as_str())
        .map(|s| s.to_string());

    let transitions: Vec<TransitionInfo> = v
        .get("execution")
        .and_then(|e| e.get("transitions"))
        .and_then(|t| t.as_array())
        .map(|arr| {
            arr.iter()
                .map(|t| {
                    let program_id = t
                        .get("program_id")
                        .and_then(|p| p.as_str())
                        .unwrap_or("?")
                        .to_string();
                    let function_name = t
                        .get("function_name")
                        .and_then(|f| f.as_str())
                        .unwrap_or("?")
                        .to_string();
                    let inputs: Vec<String> = t
                        .get("inputs")
                        .and_then(|i| i.as_array())
                        .map(|iarr| {
                            iarr.iter()
                                .filter_map(|i| {
                                    i.get("value")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                })
                                .collect()
                        })
                        .unwrap_or_default();
                    TransitionInfo {
                        program_id,
                        function_name,
                        inputs,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let deployed_program = v
        .get("deployment")
        .and_then(|d| d.get("program_id"))
        .and_then(|p| p.as_str())
        .map(|s| s.to_string());

    Ok(TransactionSummary {
        tx_type,
        status,
        owner,
        transitions,
        deployed_program,
    })
}

#[derive(Debug, PartialEq)]
pub struct TransactionSummary {
    pub tx_type: String,
    pub status: String,
    pub owner: Option<String>,
    pub transitions: Vec<TransitionInfo>,
    pub deployed_program: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct TransitionInfo {
    pub program_id: String,
    pub function_name: String,
    pub inputs: Vec<String>,
}

pub async fn run(node: &str, tx_id: &str, deep: bool) -> Result<()> {
    let client = AleoClient::new(node)?;

    println!("=== Aleo Verify ===\n");
    println!("🔍 Transaction: {tx_id}");

    let raw = client.network.fetch_transaction(tx_id).await?;
    let summary = parse_transaction_summary(&raw)?;

    println!("📋 Type:   {}", summary.tx_type);
    println!("📋 Status: {}", summary.status);

    if let Some(ref owner) = summary.owner {
        println!("👤 Owner:  {}", &owner[..20.min(owner.len())]);
    }

    for (i, t) in summary.transitions.iter().enumerate() {
        println!(
            "\n🔧 Transition #{i}: {}::{}",
            t.program_id, t.function_name
        );
        for val in &t.inputs {
            println!("   Input: {val}");
        }
    }

    if let Some(ref pid) = summary.deployed_program {
        println!("\n📦 Deployed program: {pid}");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_execution_tx() {
        let raw = r#"{
            "type": "execute",
            "status": "confirmed",
            "owner": {"address": "aleo1abc123"},
            "execution": {
                "transitions": [{
                    "program_id": "credits.aleo",
                    "function_name": "transfer_public",
                    "inputs": [{"value": "aleo1abc123", "type": "address"}, {"value": "1000000u64", "type": "u64"}]
                }]
            }
        }"#;
        let summary = parse_transaction_summary(raw).unwrap();
        assert_eq!(summary.tx_type, "execute");
        assert_eq!(summary.status, "confirmed");
        assert_eq!(summary.owner, Some("aleo1abc123".into()));
        assert_eq!(summary.transitions.len(), 1);
        assert_eq!(summary.transitions[0].program_id, "credits.aleo");
        assert_eq!(summary.transitions[0].function_name, "transfer_public");
        assert_eq!(summary.transitions[0].inputs[0], "aleo1abc123");
        assert_eq!(summary.transitions[0].inputs[1], "1000000u64");
        assert!(summary.deployed_program.is_none());
    }

    #[test]
    fn test_parse_deployment_tx() {
        let raw = r#"{
            "type": "deploy",
            "status": "confirmed",
            "deployment": {"program_id": "hello.aleo"},
            "owner": {"address": "aleo1xyz789"}
        }"#;
        let summary = parse_transaction_summary(raw).unwrap();
        assert_eq!(summary.deployed_program, Some("hello.aleo".into()));
        assert_eq!(summary.tx_type, "deploy");
        assert_eq!(summary.owner, Some("aleo1xyz789".into()));
    }

    #[test]
    fn test_parse_minimal_tx() {
        let raw = r#"{"type": "execute", "status": "confirmed"}"#;
        let summary = parse_transaction_summary(raw).unwrap();
        assert_eq!(summary.tx_type, "execute");
        assert!(summary.owner.is_none());
        assert!(summary.transitions.is_empty());
        assert!(summary.deployed_program.is_none());
    }

    #[test]
    fn test_parse_malformed_json() {
        let result = parse_transaction_summary("not json");
        assert!(result.is_err());
    }
}
