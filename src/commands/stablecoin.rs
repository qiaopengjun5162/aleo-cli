use aleo_rust_sdk::{sealance::SealanceMerkleTree, AleoHttpClient};
use anyhow::Result;
use snarkvm::prelude::TestnetV0;

pub async fn run(node: &str, stablecoin: &str, prove: Option<&str>) -> Result<()> {
    let client = AleoHttpClient::new(node)?;

    println!("===== Stablecoin: {stablecoin} =====\n");

    // Fetch Merkle tree from Provable API
    println!("📥 Fetching freeze list Merkle tree...");
    let tree_strings = client.fetch_stablecoin_merkle_tree(stablecoin).await?;
    println!("✅ Got {} elements\n", tree_strings.len());

    // Convert to fields
    let tree_fields = SealanceMerkleTree::convert_tree_to_fields::<TestnetV0>(&tree_strings)?;
    // The tree from the API is a flat array: leaves (sorted addresses as fields)
    // The number of leaves = (len + 1) / 2
    let num_leaves = tree_fields.len().div_ceil(2);
    println!(
        "🌳 Tree size: {} nodes ({} leaves)\n",
        tree_fields.len(),
        num_leaves
    );

    // If --prove is set, generate exclusion proof
    if let Some(target_address) = prove {
        println!("🛡️  Generating exclusion proof for: {target_address}");
        println!("   Depth: 15\n");

        let (left_path, right_path, left_idx, right_idx) =
            SealanceMerkleTree::generate_exclusion_proof::<TestnetV0>(
                &tree_strings,
                target_address,
                15,
            )?;

        println!("📌 Left leaf index:  {left_idx}");
        println!("📌 Right leaf index: {right_idx}");
        println!();
        println!("📋 Left sibling path ({} levels):", left_path.len());
        for (i, f) in left_path.iter().enumerate() {
            println!("   [{i}] {}", f);
        }
        println!();
        println!("📋 Right sibling path ({} levels):", right_path.len());
        for (i, f) in right_path.iter().enumerate() {
            println!("   [{i}] {}", f);
        }
        println!();

        // Format as Aleo-compatible proof
        let formatted = SealanceMerkleTree::format_merkle_proof(&[
            (left_path, left_idx),
            (right_path, right_idx),
        ]);
        println!("🔑 Formatted exclusion proof:");
        println!("{formatted}");
        println!();

        // Also show the root
        let leaves = SealanceMerkleTree::generate_leaves::<TestnetV0>(&tree_strings, 15)?;
        let tree = SealanceMerkleTree::build_tree::<TestnetV0>(&leaves)?;
        println!("🌿 Merkle root: {}", tree.last().unwrap());
    }

    // Show first few and last few elements of the tree
    println!("─── Tree sample (first 3, last 3) ───");
    let show_count = tree_strings.len().min(3);
    for i in 0..show_count {
        println!(
            "   [{i}] {}",
            &tree_strings[i][..tree_strings[i].len().min(40)]
        );
    }
    println!("   ...");
    for i in tree_strings.len().saturating_sub(3)..tree_strings.len() {
        println!(
            "   [{i}] {}",
            &tree_strings[i][..tree_strings[i].len().min(40)]
        );
    }

    println!("\n===== Done =====");
    Ok(())
}
