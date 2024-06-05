use anchor_lang::prelude::*;

declare_id!("pExcmMBLztfuENuvZbN7MmbpG1yV93hMcMz9t1Y7Tyh");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
