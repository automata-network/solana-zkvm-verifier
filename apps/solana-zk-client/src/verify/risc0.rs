pub const RISCZERO_GROTH16_VERIFY_INSTRUCTION_DISCRIMINATOR: [u8; 8] =
    [133, 161, 141, 48, 120, 198, 88, 150];

pub fn risc0_verify_instruction_data(
    proof_bytes: &[u8],
    program_image_id: [u8; 32],
    output_digest: [u8; 32]
) -> Vec<u8> {
    let mut instruction_data = Vec::new();
    instruction_data.extend_from_slice(&RISCZERO_GROTH16_VERIFY_INSTRUCTION_DISCRIMINATOR);
    instruction_data.extend_from_slice(proof_bytes);
    instruction_data.extend_from_slice(&program_image_id);
    instruction_data.extend_from_slice(&output_digest);
    instruction_data
}