use anchor_client::{
    solana_sdk::{
        signature::{Keypair, read_keypair_file},
        pubkey::Pubkey,
    },
    solana_client::nonblocking::rpc_client::RpcClient,
};
use anyhow::Result;
use super::deploy_program;

/// Deploy the RISC0 Groth16 verifier program to the test validator
pub async fn deploy_risc0_groth16_verifier(
    payer: &Keypair,
    client: &RpcClient,
) -> Result<Pubkey> {
    let groth16_verifier_path = format!(
        "{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        "src/zkvm/risc0/so/groth_16_verifier.so"
    );

    let groth16_verifier_keypair_path = format!(
        "{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        "src/zkvm/risc0/so/groth_16_verifier-test-keypair.json"
    );
    
    // Use the centralized deployment function
    deploy_program(
        payer, 
        client, 
        &groth16_verifier_path,
        read_keypair_file(&groth16_verifier_keypair_path).unwrap(),
    ).await
}
