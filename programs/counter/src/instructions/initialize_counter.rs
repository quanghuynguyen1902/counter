use anchor_lang::prelude::*;
use crate::states::*;

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeCounter>) -> Result<()> {
    Ok(())
}