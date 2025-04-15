// use borsh::{BorshDeserialize, BorshSerialize};

// // 
// #[derive(BorshDeserialize, BorshSerialize)]
// pub struct SP1Groth16Proof {
//     pub proof: Vec<u8>,
//     /// SHA256 of the public inputs
//     pub sp1_public_inputs_hash: Vec<u8>,
// }

pub fn sp1_groth16_verify_instruction_data(
    proof_bytes: &[u8],
    program_vkey: [u8; 32],
    output_digest: [u8; 32]
) -> Vec<u8> {
    // TODO

    vec![]
}