#[cfg(test)]
mod test_solana_zk;

pub mod zkvm;

use anchor_client::Cluster;
use anchor_client::solana_client::nonblocking::rpc_client::RpcClient;
use anchor_client::solana_sdk::{
    signature::Keypair,
    commitment_config::CommitmentConfig
};
use solana_zk_client::SolanaZkClient;

pub fn setup<'a>(payer: &'a Keypair) -> SolanaZkClient<&'a Keypair> {
    SolanaZkClient::new(
        payer,
        Some(Cluster::Localnet)
    )
}

/// Get an RpcClient for program deployment
pub fn get_rpc_client() -> RpcClient {
    RpcClient::new_with_commitment(
        "http://localhost:8899".to_string(),
        CommitmentConfig::confirmed()
    )
}