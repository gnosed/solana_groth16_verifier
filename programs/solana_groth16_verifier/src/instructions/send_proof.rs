use crate::state::Circuit::*;
use anchor_lang::prelude::*;

pub fn send_proof(ctx: Context<SendProof>, proof: [u8; 256]) -> Result<()> {
    let circuit = &mut ctx.accounts.circuit;

    circuit.verify_proof(&proof)
}

#[derive(Accounts)]
pub struct SendProof<'info> {
    #[account(mut)]
    pub circuit: Account<'info, Circuit>,
}
