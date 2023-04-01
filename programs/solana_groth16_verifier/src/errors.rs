use anchor_lang::error_code;

#[error_code]
pub enum Groth16VerifierError {
    IncompatibleVerifyingKeyWithNrPublicInputs,
    ProofVerificationFailed,
    InvalidG1Length,
    InvalidG2Length,
    InvalidPublicInputsLength,
}
