use anchor_lang::prelude::*;
use instructions::*;

use state::*;

pub mod instructions;
pub mod public_inputs;
pub mod state;
pub mod verifying_key;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_groth16_verifier {
    use super::*;

    pub fn setup_circuit(ctx: Context<SetupCircuit>) -> Result<()> {
        instructions::setup_circuit::setup_circuit()
    }
}
