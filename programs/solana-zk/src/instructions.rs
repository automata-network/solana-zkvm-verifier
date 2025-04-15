use super::state::{Counter, ZkvmVerifier};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{bpf_loader_upgradeable, system_program};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    pub payer: Signer<'info>,

    #[account(
        init, 
        payer = payer, 
        space = 8 + 8,
        seeds = [b"counter"],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    /// CHECK: This is the program data account for the current program
    #[account(
        constraint = program_data.key() == Pubkey::find_program_address(
            &[crate::ID.as_ref()],
            &bpf_loader_upgradeable::id()
        ).0
    )]
    pub program_data: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(
    zkvm_selector: u64
)]
pub struct AddZkvmVerifier<'info> {
    #[account(mut, signer)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub counter: Account<'info, Counter>,

    #[account(
        init,
        payer = owner,
        space = 8 + 8 + 32 + 1,
        seeds = [
            b"zkvm_verifier",
            zkvm_selector.to_le_bytes().as_ref(),
            zkvm_verifier_program.key().as_ref(),
        ],
        bump,
    )]
    pub zkvm_verifier_account: Account<'info, ZkvmVerifier>,

    /// CHECK: This is the program data account for the current program
    #[account(
        constraint = program_data.key() == Pubkey::find_program_address(
            &[crate::ID.as_ref()],
            &bpf_loader_upgradeable::id()
        ).0
    )]
    pub program_data: AccountInfo<'info>,

    /// CHECK: This is the address of the ZKVM Verifier Program. Currently, there isn't any defined standards to structure the program.
    pub zkvm_verifier_program: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    zkvm_selector: u64
)]
pub struct UpdateZkvmVerifierConfig<'info> {
    #[account(mut, signer)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"zkvm_verifier",
            zkvm_selector.to_le_bytes().as_ref(),
            zkvm_verifier_program.key().as_ref(),
        ],
        bump,
    )]
    pub zkvm_verifier_account: Account<'info, ZkvmVerifier>,

    /// CHECK: This is the program data account for the current program
    #[account(
        constraint = program_data.key() == Pubkey::find_program_address(
            &[crate::ID.as_ref()],
            &bpf_loader_upgradeable::id()
        ).0
    )]
    pub program_data: AccountInfo<'info>,

    /// CHECK: This is the address of the ZKVM Verifier Program. Currently, there isn't any defined standards to structure the program.
    pub zkvm_verifier_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(
    zkvm_selector: u64,
    zk_verify_instruction_data: Vec<u8>
)]
pub struct VerifyZkProof<'info> {
    #[account(
        seeds = [
            b"zkvm_verifier",
            zkvm_selector.to_le_bytes().as_ref(),
            zkvm_verifier_program.key().as_ref(),
        ],
        bump,
    )]
    pub zkvm_verifier_account: Account<'info, ZkvmVerifier>,

    /// CHECK: This is the address of the ZKVM Verifier Program. Currently, there isn't any defined standards to structure the program.
    pub zkvm_verifier_program: AccountInfo<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
