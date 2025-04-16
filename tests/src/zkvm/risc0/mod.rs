use anchor_client::{
    solana_sdk::{
        signature::Keypair,
        pubkey::Pubkey,
    },
    solana_client::nonblocking::rpc_client::RpcClient,
};
use anyhow::Result;
use super::deploy_program;

/// The path to the Groth16 verifier program binary
pub const GROTH16_VERIFIER_SO_PATH: &str = "src/zkvm/risc0/so/groth_16_verifier.so";

/// Deploy the RISC0 Groth16 verifier program to the test validator
pub async fn deploy_risc0_groth16_verifier(
    payer: &Keypair,
    client: &RpcClient,
) -> Result<Pubkey> {
    // Use the centralized deployment function
    deploy_program(payer, client, GROTH16_VERIFIER_SO_PATH).await
}
