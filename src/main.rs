use std::str::FromStr;
use tokio;
use fuels::{
    prelude::*,
    signers::fuel_crypto::SecretKey,
};

#[tokio::main]
async fn main() {
        let provider = Provider::connect("node-beta-2.fuel.network").await.unwrap();
        let secret = SecretKey::from_str("a1447cd75accc6b71a976fd3401a1f6ce318d27ba660b0315ee6ac347bf39568").unwrap();

        let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));
        dbg!(wallet.address().to_string());
}

