use fuels::{
    client::FuelClient, fuel_node::FuelService, prelude::*, signers::fuel_crypto::SecretKey,
    types::error,
};
use std::str::FromStr;

async fn unlock_wallet() -> WalletUnlocked {
    let provider = Provider::connect("node-beta-2.fuel.network").await.unwrap();
    let secret =
        SecretKey::from_str("a1447cd75accc6b71a976fd3401a1f6ce318d27ba660b0315ee6ac347bf39568")
            .unwrap();

    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));
    dbg!(wallet.address().to_string());
    wallet
}

async fn run_baby_node() -> FuelClient {
    //run the fuel node
    let server = FuelService::new_node(Config::local_node()).await.unwrap();

    //create a client to talk to the node
    let client = FuelClient::from(server.bound_address);
    client
}

#[tokio::test]
async fn can_unlock_wallet() {
    let unlocked = unlock_wallet().await;
    assert!(unlocked.address().to_string().len() > 0);
}

#[tokio::test]
async fn can_run_baby_node() {
    let client = run_baby_node().await;
    //check that client is connected
    let status = client.health().await.unwrap();
    assert_eq!(status, true);
}
