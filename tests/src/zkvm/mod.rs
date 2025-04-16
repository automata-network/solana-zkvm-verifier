use anchor_client::{
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        bpf_loader_upgradeable,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
};
use anyhow::Result;
use std::{fs, ops::Add};

pub mod risc0;

/// Deploy a program to the test validator using its binary (.so) file
pub async fn deploy_program(
    payer: &Keypair, 
    client: &RpcClient, 
    program_path: &str,
    program_keypair: Keypair
) -> Result<Pubkey> {
    // Load the program binary
    let program_data = fs::read(program_path)?;

    // Create a new program keypair
    let program_id = program_keypair.pubkey();

    // Create a buffer account
    let buffer_keypair = Keypair::new();
    let buffer_pubkey = buffer_keypair.pubkey();

    // Calculate the required size for the program
    let program_len = program_data.len();

    // TEMP: Adding one sol to prevent insufficient funds for rent error
    // why is this not giving me the correct rent exemption amount?
    let minimum_rent = client
        .get_minimum_balance_for_rent_exemption(program_len)
        .await?
        .add(1_000_000);

    // Create the buffer account
    let create_buffer_ix = bpf_loader_upgradeable::create_buffer(
        &payer.pubkey(),
        &buffer_pubkey,
        &payer.pubkey(),
        minimum_rent,
        program_len as usize,
    )?;

    let create_buffer_tx = Transaction::new_signed_with_payer(
        &create_buffer_ix,
        Some(&payer.pubkey()),
        &[payer, &buffer_keypair],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&create_buffer_tx).await?;

    // Write program data to buffer in chunks
    const CHUNK_SIZE: usize = 900; // Solana has a limit on transaction size

    for (i, chunk) in program_data.chunks(CHUNK_SIZE).enumerate() {
        let offset = i * CHUNK_SIZE;

        let write_ix = bpf_loader_upgradeable::write(
            &buffer_pubkey,
            &payer.pubkey(),
            offset as u32,
            chunk.to_vec(),
        );

        let write_tx = Transaction::new_signed_with_payer(
            &[write_ix],
            Some(&payer.pubkey()),
            &[payer],
            client.get_latest_blockhash().await?,
        );

        client.send_and_confirm_transaction(&write_tx).await?;
    }

    // Deploy the program from the buffer
    let deploy_ix = bpf_loader_upgradeable::deploy_with_max_program_len(
        &payer.pubkey(),
        &program_id,
        &buffer_pubkey,
        &payer.pubkey(),
        minimum_rent,
        program_len as usize,
    )?;

    let deploy_tx = Transaction::new_signed_with_payer(
        &deploy_ix,
        Some(&payer.pubkey()),
        &[payer, &program_keypair],
        client.get_latest_blockhash().await?,
    );

    client.send_and_confirm_transaction(&deploy_tx).await?;

    // Return the deployed program ID
    Ok(program_id)
}
