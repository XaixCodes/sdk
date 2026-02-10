use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod instructions;

use instructions::*;

declare_id!("Xaix1111111111111111111111111111111111111");

#[program]
pub mod xaix {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn register_agent(ctx: Context<RegisterAgent>) -> Result<()> {
        register_agent::handler(ctx)
    }

    pub fn complete_task(ctx: Context<CompleteTask>) -> Result<()> {
        complete_task::handler(ctx)
    }
}
