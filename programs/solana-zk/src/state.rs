use anchor_lang::prelude::*;

/// This account keeps track of the number of zkVM verifier configured in the program
#[account]
pub struct Counter {
    pub count: u64
}

/// This account stores the Verifier program ID and the corresponding vkey of the zkVM Program
#[account]
pub struct ZkvmVerifier {
    pub zkvm_selector: u64,
    pub zkvm_program_id: Pubkey,
    pub frozen: bool
}