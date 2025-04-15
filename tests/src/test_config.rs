use super::*;
use anchor_client::solana_sdk::{
    signature::read_keypair_file,
    signer::{keypair::Keypair, Signer},
};
use solana_zk_client::{selector::ZkvmSelectorType, RISC0_VERIFIER_ROUTER_ID};

#[tokio::test]
async fn test_initialize() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    let client = setup(&payer);
    client.initialize().await.expect("Failed to initialize");

    // Fetch the counter account data
    let counter_account = client
        .program()
        .account::<solana_zk::state::Counter>(client.counter())
        .await
        .expect("Failed to fetch counter account");

    assert_eq!(counter_account.count, 0);
}

#[tokio::test]
async fn test_config_zkvm_verifier() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    let client = setup(&payer);

    // Fetch the zkvm verifier PDA
    let zkvm_selector = ZkvmSelectorType::RiscZero;
    let (zkvm_verifier_config_pda_id, _) = client
        .derive_zkvm_verifier_pda(zkvm_selector.to_u64(), &zkvm_selector.to_zkvm_verifier_id());

    client.add_zk_verifier_program(zkvm_selector).await.unwrap();

    // Fetch the counter account data
    let counter_account = client
        .program()
        .account::<solana_zk::state::Counter>(client.counter())
        .await
        .expect("Failed to fetch counter account");

    // Fetch the Verifier config PDA account data
    let zkvm_verifier_config_pda = client
        .program()
        .account::<solana_zk::state::ZkvmVerifier>(zkvm_verifier_config_pda_id)
        .await
        .expect("Failed to fetch zkvm verifier config account");

    assert_eq!(counter_account.count, 1);
    assert_eq!(
        zkvm_verifier_config_pda.zkvm_program_id,
        RISC0_VERIFIER_ROUTER_ID
    );
    assert_eq!(
        zkvm_verifier_config_pda.zkvm_selector,
        zkvm_selector.to_u64()
    );
    assert_eq!(zkvm_verifier_config_pda.frozen, false);
}
