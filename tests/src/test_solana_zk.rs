use super::*;
use crate::zkvm::risc0::deploy_risc0_groth16_verifier;
use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use solana_zk_client::selector::ZkvmSelectorType;

#[tokio::test]
async fn test_solana_zk_program() {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();
    let client = setup(&payer);

    println!("====== TESTING SOLANA ZK PROGRAM ===");

    println!("====== test_initialize ======");
    test_initialize(&client).await;
    println!("====== test_initialize ====== DONE");

    println!("====== test_config_risc0 ======");
    let risc0_pubkey = test_config_risc0(&client, &payer).await;
    println!("====== test_config_risc0 ====== DONE");

    println!("====== test_verify_risc0_proof ======");
    test_verify_risc0_proof(&client, &risc0_pubkey).await;
    println!("====== test_verify_risc0_proof ====== DONE");
}

async fn test_initialize(client: &SolanaZkClient<&Keypair>) {
    client.initialize().await.expect("Failed to initialize");

    // Fetch the counter account data
    let counter_account = client
        .program()
        .account::<solana_zk::state::Counter>(client.counter())
        .await
        .expect("Failed to fetch counter account");

    assert_eq!(counter_account.count, 0);
}

async fn test_config_risc0(client: &SolanaZkClient<&Keypair>, payer: &Keypair) -> Pubkey {
    // deploy the RiscZero Groth16 Verifier program
    let rpc_client = get_rpc_client();
    let zkvm_verifier_program_id = deploy_risc0_groth16_verifier(&payer, &rpc_client)
        .await
        .expect("Failed to deploy Risc0 Groth16 Verifier program");

    // Fetch the zkvm verifier PDA
    let zkvm_selector = ZkvmSelectorType::RiscZero;
    let (zkvm_verifier_config_pda_id, _) =
        client.derive_zkvm_verifier_pda(zkvm_selector.to_u64(), &zkvm_verifier_program_id);

    client
        .add_zk_verifier_program(zkvm_selector, Some(zkvm_verifier_program_id))
        .await
        .unwrap();

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
        zkvm_verifier_program_id
    );
    assert_eq!(
        zkvm_verifier_config_pda.zkvm_selector,
        zkvm_selector.to_u64()
    );
    assert_eq!(zkvm_verifier_config_pda.frozen, false);

    zkvm_verifier_program_id
}

async fn test_verify_risc0_proof(client: &SolanaZkClient<&Keypair>, risc0_program_id: &Pubkey) {
    let program_vkey: [u8; 32] = [
        194, 234, 254, 27, 160, 22, 16, 243, 183, 18, 129, 249, 221, 50, 128, 179, 61, 151, 55, 11,
        182, 141, 58, 218, 41, 37, 211, 145, 190, 36, 94, 16,
    ];
    let output_digest: [u8; 32] = [
        82, 214, 11, 39, 59, 213, 203, 56, 126, 18, 201, 48, 106, 142, 95, 222, 29, 78, 90, 31,
        203, 21, 88, 64, 76, 137, 82, 59, 91, 242, 160, 174,
    ];
    let proof_bytes = hex::decode("1850aa52559f1d4a858a48b788b52bdd963888e29465a59ca4dace241ad1aeef2b1796d0acb6ea9f4d77a60a0555f28c85867e62b91ac8d0473ff017c88883da077c6be0d1140a77f0ab695679470472cc32f55ebdcf735e9d52ff4a53d3b685020772e77e8e94578796fd6cc122420a77c1c0ba8dff1c6e07e53e30da46d483147732f37ffb72fda399256a551beb49da688ea7cbdcf268fbc15695c3db42a40569e5093c75654a1390cb1fe9c57c360a8f338f66d61ae1115d4584faecc36f238a9eb4cfecea8d3e4995a354dbe5c4bc12db6a12da41e376931548110fb3c008c01d08cf9e8afb7fe661befbb5afce139c9a1ba1b6c10562645ce60954ab48").unwrap();

    client
        .verify_zkvm_proof(
            ZkvmSelectorType::RiscZero,
            Some(risc0_program_id.clone()),
            program_vkey,
            output_digest,
            proof_bytes.as_slice(),
        )
        .await
        .unwrap();
}
