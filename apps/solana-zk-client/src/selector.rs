use anchor_client::solana_sdk::pubkey::Pubkey;
use super::{
    RISC0_VERIFIER_ROUTER_ID,
    SUCCINCT_SPI_VERIFIER_ID
};

#[derive(Clone, Copy, Debug)]
#[repr(u64)]
pub enum ZkvmSelectorType {
    RiscZero = 1,
    Succinct = 2 
}

impl ZkvmSelectorType {
    pub fn to_u64(&self) -> u64 {
        match self {
            ZkvmSelectorType::RiscZero => 1,
            ZkvmSelectorType::Succinct => 2,
        }
    }

    pub fn to_zkvm_verifier_id(&self) -> Pubkey {
        match self {
            ZkvmSelectorType::RiscZero => RISC0_VERIFIER_ROUTER_ID,
            ZkvmSelectorType::Succinct => SUCCINCT_SPI_VERIFIER_ID,
        }
    }
}