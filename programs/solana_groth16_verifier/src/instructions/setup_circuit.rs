// use crate::public_inputs::PUBLIC_INPUTS;
// use crate::state::circuit::*;
// use crate::verifying_key::VERIFYING_KEY;
// use anchor_lang::prelude::*;

// pub fn setup_circuit(ctx: Context<SetupCircuit>) -> Result<()> {
//     ctx.accounts.circuit.setup(VERIFYING_KEY, PUBLIC_INPUTS)
// }

// #[derive(Accounts)]
// pub struct SetupCircuit<'info> {
//     #[account(init, payer = owner, space = Circuit::MAXIMUM_SIZE )]
//     pub circuit: Account<'info, Circuit>,
//     #[account(mut)]
//     pub owner: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }
