use anchor_lang::prelude::*;

#[error_code]
pub enum ZkError {
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid zkvm selector")]
    InvalidZkvmSelector,

    #[msg("ZK Proof verification failed")]
    FailedZkProofVerification,

    #[msg("zkVM Program frozen")]
    ZkvmProgramFrozen,
}