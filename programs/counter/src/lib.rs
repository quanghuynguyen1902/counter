
pub mod states;

pub mod instructions;
mod utils;

use anchor_lang::prelude::*;
use instructions::*;
use states::*;

use anchor_lang::prelude::*;

declare_id!("pExcmMBLztfuENuvZbN7MmbpG1yV93hMcMz9t1Y7Tyh");

#[program]
pub mod counter {
    use super::*;
    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        initialize_counter::handler(ctx)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        increment_counter::handler(ctx)
    }
}

