#[cfg(test)]
mod test_solana_zk;

use anchor_client::solana_sdk::signature::Keypair;
use solana_zk_client::SolanaZkClient;

pub fn setup<'a>(payer: &'a Keypair) -> SolanaZkClient<&'a Keypair> {
    SolanaZkClient::new(
        payer,
        Some(anchor_client::Cluster::Localnet)
    )
}