// The verify module is always included
pub mod verify;

// TEMP
pub const RISC0_VERIFIER_ROUTER_ID: Pubkey =
    Pubkey::from_str_const("5HrF6mJAaSFdAym2xZixowzVifPyyzTuTs3viYKdjy4s");
pub const SUCCINCT_SP1_VERIFIER_ID: Pubkey =
    Pubkey::from_str_const("2LUaFQTJ7F96A5x1z5sXfbDPM2asGnrQ2hsE6zVDMhXZ");

// Other modules and imports are conditionally included
#[cfg(feature = "client")]
pub mod selector;
#[cfg(feature = "client")]
use selector::ZkvmSelectorType;

#[cfg(feature = "client")]
use verify::risc0::risc0_verify_instruction_data;
#[cfg(feature = "client")]
use verify::succinct::sp1_groth16_verify_instruction_data;

// Conditionally include client-specific imports
#[cfg(feature = "client")]
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signer::Signer, system_program,
    },
    Client, Cluster, Program,
};
#[cfg(feature = "client")]
use anyhow::{Error, Result};
use solana_zk::{accounts, instruction, ID};
#[cfg(feature = "client")]
use std::ops::Deref;

/// Client for interacting with the Solana ZK program
#[cfg(feature = "client")]
pub struct SolanaZkClient<C> {
    program: Program<C>,
    counter: Pubkey,
}

#[cfg(feature = "client")]
impl<C: Clone + Deref<Target = impl Signer>> SolanaZkClient<C> {
    /// Create a new client instance
    pub fn new(payer: C, cluster: Option<Cluster>) -> Self {
        let cluster = cluster.unwrap_or(Cluster::Localnet);
        let client = Client::new_with_options(cluster, payer, CommitmentConfig::confirmed());
        let program = client.program(ID).unwrap();

        Self {
            program,
            counter: derive_counter_pda().0,
        }
    }

    /// Initialize a new counter account for tracking ZKVM verifiers
    pub async fn initialize(&self) -> Result<String> {
        let signature = self
            .program
            .request()
            .accounts(accounts::Initialize {
                payer: self.program.payer(),
                counter: self.counter,
                system_program: system_program::ID,
            })
            .args(instruction::Initialize {})
            .send()
            .await?;

        Ok(signature.to_string())
    }

    /// Add a new ZKVM verifier program
    pub async fn add_zk_verifier_program(
        &self,
        zkvm_selector: ZkvmSelectorType,
        overwrite_zkvm_verifier_pubkey: Option<Pubkey>,
    ) -> Result<String> {
        // Ensure the payer is the program's upgrade authority
        self.require_upgrade_authority().await?;

        let zkvm_selector_u64 = zkvm_selector.to_u64();
        let zkvm_verifier_program = match overwrite_zkvm_verifier_pubkey {
            Some(pubkey) => pubkey,
            None => zkvm_selector.to_zkvm_verifier_id(),
        };

        let (verifier_account, _bump) =
            self.derive_zkvm_verifier_pda(zkvm_selector_u64, &zkvm_verifier_program);
        let (program_data, _) = Pubkey::find_program_address(
            &[ID.as_ref()],
            &solana_program::bpf_loader_upgradeable::ID,
        );

        let signature = self
            .program
            .request()
            .accounts(accounts::AddZkvmVerifier {
                owner: self.program.payer(),
                counter: self.counter,
                zkvm_verifier_account: verifier_account,
                program_data,
                zkvm_verifier_program,
                system_program: system_program::ID,
            })
            .args(instruction::AddZkVerifierProgram {
                zkvm_selector: zkvm_selector_u64,
            })
            .send()
            .await?;

        Ok(signature.to_string())
    }

    /// Update an existing ZKVM verifier program
    pub async fn update_zk_verifier_program(
        &self,
        zkvm_selector: ZkvmSelectorType,
        overwrite_zkvm_verifier_pubkey: Option<Pubkey>,
    ) -> Result<String> {
        // Ensure the payer is the program's upgrade authority
        self.require_upgrade_authority().await?;

        let zkvm_selector_u64 = zkvm_selector.to_u64();
        let zkvm_verifier_program = match overwrite_zkvm_verifier_pubkey {
            Some(pubkey) => pubkey,
            None => zkvm_selector.to_zkvm_verifier_id(),
        };

        let (verifier_account, _bump) =
            self.derive_zkvm_verifier_pda(zkvm_selector_u64, &zkvm_verifier_program);
        let (program_data, _) = Pubkey::find_program_address(
            &[ID.as_ref()],
            &solana_program::bpf_loader_upgradeable::ID,
        );

        let signature = self
            .program
            .request()
            .accounts(accounts::UpdateZkvmVerifierConfig {
                owner: self.program.payer(),
                zkvm_verifier_account: verifier_account,
                program_data,
                zkvm_verifier_program,
            })
            .args(instruction::UpdateZkVerifierProgram {
                _zkvm_selector: zkvm_selector_u64,
            })
            .send()
            .await?;

        Ok(signature.to_string())
    }

    /// Freeze a ZKVM verifier program to prevent further updates
    pub async fn freeze_zk_verifier_program(
        &self,
        zkvm_selector: ZkvmSelectorType,
        overwrite_zkvm_verifier_pubkey: Option<Pubkey>,
        freeze: bool,
    ) -> Result<String> {
        // Ensure the payer is the program's upgrade authority
        self.require_upgrade_authority().await?;

        let zkvm_selector_u64 = zkvm_selector.to_u64();
        let zkvm_verifier_program = match overwrite_zkvm_verifier_pubkey {
            Some(pubkey) => pubkey,
            None => zkvm_selector.to_zkvm_verifier_id(),
        };

        let (verifier_account, _bump) =
            self.derive_zkvm_verifier_pda(zkvm_selector_u64, &zkvm_verifier_program);
        let (program_data, _) = Pubkey::find_program_address(
            &[ID.as_ref()],
            &solana_program::bpf_loader_upgradeable::ID,
        );

        let signature = self
            .program
            .request()
            .accounts(accounts::UpdateZkvmVerifierConfig {
                owner: self.program.payer(),
                zkvm_verifier_account: verifier_account,
                program_data,
                zkvm_verifier_program,
            })
            .args(instruction::FreezeZkVerifierProgram {
                _zkvm_selector: zkvm_selector_u64,
                freeze,
            })
            .send()
            .await?;

        Ok(signature.to_string())
    }

    /// Interface for verifying ZKVM proofs (to be implemented by user)
    pub async fn verify_zkvm_proof(
        &self,
        zkvm_selector: ZkvmSelectorType,
        overwrite_zkvm_verifier_pubkey: Option<Pubkey>,
        program_vkey: [u8; 32],
        output_digest: [u8; 32],
        proof_data: &[u8],
    ) -> Result<String> {
        let zkvm_selector_u64 = zkvm_selector.to_u64();
        let zkvm_verifier_program = match overwrite_zkvm_verifier_pubkey {
            Some(pubkey) => pubkey,
            None => zkvm_selector.to_zkvm_verifier_id(),
        };

        let (verifier_account, _bump) =
            self.derive_zkvm_verifier_pda(zkvm_selector_u64, &zkvm_verifier_program);

        // Check if verifier exists
        let verifier = self
            .program
            .account::<solana_zk::state::ZkvmVerifier>(verifier_account)
            .await?;

        // Check if verifier is frozen
        if verifier.frozen {
            return Err(Error::msg("ZKVM verifier is frozen"));
        }

        let instruction_data: Vec<u8> = match zkvm_selector {
            ZkvmSelectorType::RiscZero => {
                risc0_verify_instruction_data(&proof_data, program_vkey, output_digest)
            }
            ZkvmSelectorType::Succinct => {
                sp1_groth16_verify_instruction_data(&proof_data, program_vkey, output_digest)
            }
        };

        let signature = self
            .program
            .request()
            .accounts(accounts::VerifyZkProof {
                zkvm_verifier_account: verifier_account,
                zkvm_verifier_program,
                system_program: system_program::ID,
            })
            .args(instruction::VerifyZkvmProof {
                _zkvm_selector: zkvm_selector_u64,
                zk_verify_instruction_data: instruction_data,
            })
            .send()
            .await?;

        Ok(signature.to_string())
    }

    /// Helper method to derive the PDA for a ZKVM verifier account
    pub fn derive_zkvm_verifier_pda(
        &self,
        zkvm_selector: u64,
        zkvm_verifier_program: &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                b"zkvm_verifier",
                zkvm_selector.to_le_bytes().as_ref(),
                zkvm_verifier_program.as_ref(),
            ],
            &ID,
        )
    }

    /// Get the program instance
    pub fn program(&self) -> &Program<C> {
        &self.program
    }

    /// Get the counter pubkey
    pub fn counter(&self) -> Pubkey {
        self.counter
    }

    /// Get the payer pubkey
    pub fn payer(&self) -> Pubkey {
        self.program.payer()
    }

    /// Check if the current payer is the program's upgrade authority
    pub async fn is_upgrade_authority(&self) -> Result<bool> {
        let (program_data, _) = Pubkey::find_program_address(
            &[ID.as_ref()],
            &solana_program::bpf_loader_upgradeable::ID,
        );

        // Fetch the program data account
        let program_data_account = self.program.rpc().get_account(&program_data).await?;

        // Parse the upgrade authority from the account data
        let data = &program_data_account.data;
        if data[12] == 1 {
            let mut authority = [0u8; 32];
            authority.copy_from_slice(&data[13..45]);
            let authority_pubkey = Pubkey::new_from_array(authority);

            return Ok(&authority_pubkey == &self.program.payer());
        }

        Ok(false)
    }

    /// Require that the current payer is the program's upgrade authority
    pub async fn require_upgrade_authority(&self) -> Result<()> {
        if !self.is_upgrade_authority().await? {
            return Err(Error::msg(
                "Current payer is not the program's upgrade authority",
            ));
        }
        Ok(())
    }
}

/// Helper method to derive the PDA for the Counter
#[cfg(feature = "client")]
fn derive_counter_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"counter"], &ID)
}
