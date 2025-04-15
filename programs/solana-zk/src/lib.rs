#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
pub mod errors;
pub mod instructions;
pub mod state;

use errors::*;
use instructions::*;

declare_id!("3rp28FnaSDUsrwDHiggLFY12dVKvRovNbSs8iAKEFKmv");

#[program]
pub mod solana_zk {
    use anchor_lang::solana_program::instruction::Instruction;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Check if the owner is the program's upgrade authority
        if !check_program_upgrade_authority(&ctx.accounts.program_data, &ctx.accounts.payer.key()) {
            return err!(ZkError::Unauthorized);
        }
        
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;

        Ok(())
    }

    pub fn add_zk_verifier_program(ctx: Context<AddZkvmVerifier>, zkvm_selector: u64) -> Result<()> {
        // Check if the owner is the program's upgrade authority
        if !check_program_upgrade_authority(&ctx.accounts.program_data, &ctx.accounts.owner.key()) {
            return err!(ZkError::Unauthorized);
        }

        // Increment the counter
        ctx.accounts.counter.count += 1;

        // Validate that selector matches current count
        if zkvm_selector as u64 != ctx.accounts.counter.count {
            return err!(ZkError::InvalidZkvmSelector);
        }

        // Update state
        let zkvm_verifier = &mut ctx.accounts.zkvm_verifier_account;
        zkvm_verifier.zkvm_selector = zkvm_selector;
        zkvm_verifier.zkvm_program_id = ctx.accounts.zkvm_verifier_program.key();
        zkvm_verifier.frozen = false;

        Ok(())
    }

    pub fn update_zk_verifier_program(
        ctx: Context<UpdateZkvmVerifierConfig>,
        _zkvm_selector: u64,
    ) -> Result<()> {
        // Check if the owner is the program's upgrade authority
        if !check_program_upgrade_authority(&ctx.accounts.program_data, &ctx.accounts.owner.key()) {
            return err!(ZkError::Unauthorized);
        }

        let zkvm_verifier = &mut ctx.accounts.zkvm_verifier_account;

        zkvm_verifier.zkvm_program_id = ctx.accounts.zkvm_verifier_program.key();

        Ok(())
    }

    pub fn freeze_zk_verifier_program(
        ctx: Context<UpdateZkvmVerifierConfig>,
        _zkvm_selector: u64,
        freeze: bool,
    ) -> Result<()> {
        // Check if the owner is the program's upgrade authority
        if !check_program_upgrade_authority(&ctx.accounts.program_data, &ctx.accounts.owner.key()) {
            return err!(ZkError::Unauthorized);
        }

        let zkvm_verifier = &mut ctx.accounts.zkvm_verifier_account;
        zkvm_verifier.frozen = freeze;

        Ok(())
    }

    pub fn verify_zkvm_proof(
        ctx: Context<VerifyZkProof>,
        _zkvm_selector: u64,
        zk_verify_instruction_data: Vec<u8>,
    ) -> Result<()> {
        // Step 1: Check zkvm selector matches with the expected zkvm_verifier_program
        let zkvm_verifier = &ctx.accounts.zkvm_verifier_account;

        // Step 2: Check if the zkvm_verifier_program is frozen
        if zkvm_verifier.frozen {
            return err!(ZkError::ZkvmProgramFrozen);
        }

        // Step 3: Perform CPI to zkvm_verifier_program
        let verify_cpi_context = CpiContext::new(
            ctx.accounts.zkvm_verifier_program.to_account_info(),
            vec![ctx.accounts.system_program.to_account_info()],
        );

        invoke(
            &Instruction {
                program_id: ctx.accounts.zkvm_verifier_program.key(),
                accounts: verify_cpi_context.to_account_metas(None),
                data: zk_verify_instruction_data,
            },
            &[ctx.accounts.system_program.to_account_info()],
        )?;

        Ok(())
    }
}

// Helper function to check if a pubkey matches the program's upgrade authority
fn check_program_upgrade_authority(
    program_data_info: &AccountInfo,
    owner_key: &Pubkey,
) -> bool {
    // Parse the program data account
    let data = program_data_info.try_borrow_data().unwrap();

    // The upgrade authority is at offset 13 (1 byte for is_initialized + 8 bytes for slot + 4 bytes for upgrade authority present)
    // If the upgrade authority is present (byte at offset 12 is 1), then the next 32 bytes are the upgrade authority pubkey
    if data[12] == 1 {
        let mut authority = [0u8; 32];
        authority.copy_from_slice(&data[13..45]);
        let authority_pubkey = Pubkey::new_from_array(authority);

        return &authority_pubkey == owner_key;
    }

    false
}