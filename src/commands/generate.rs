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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_output_format() {
        let mut rng = TestRng::default();
        let account = AleoAccount::new_random(&mut rng).unwrap();
        let pk = account.private_key_str();
        let vk = account.view_key.to_string();
        let addr = account.address_str();

        // Private key must be the bech32m-encoded string starting with APrivateKey1
        assert!(pk.starts_with("APrivateKey1"), "private key prefix: {pk}");
        assert_eq!(pk.len(), 59, "private key length: {pk}");

        // View key starts with AViewKey1
        assert!(vk.starts_with("AViewKey1"), "view key prefix: {vk}");

        // Address starts with aleo1
        assert!(addr.starts_with("aleo1"), "address prefix: {addr}");
        assert_eq!(addr.len(), 63, "address length: {addr}");
    }

    #[test]
    fn test_generate_different_each_time() {
        let mut rng = TestRng::default();
        let a1 = AleoAccount::new_random(&mut rng).unwrap();
        let a2 = AleoAccount::new_random(&mut rng).unwrap();
        assert_ne!(a1.private_key_str(), a2.private_key_str());
        assert_ne!(a1.address_str(), a2.address_str());
    }
}
