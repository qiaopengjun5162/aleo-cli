use aleo_rust_sdk::AleoAccount;
use anyhow::Result;
use snarkvm::prelude::TestRng;

pub fn run() -> Result<()> {
    let mut rng = TestRng::default();
    let account = AleoAccount::new_random(&mut rng)?;

    println!("=== New Aleo Account ===");
    println!("Private Key  {}", account.private_key_str());
    println!("   View Key  {}", account.view_key);
    println!("    Address  {}", account.address_str());
    println!();
    println!("⚠️  Save the private key securely! This is the only time it will be shown.");
    Ok(())
}
