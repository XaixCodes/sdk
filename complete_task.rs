use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub agent: Account<'info, Agent>,
}

pub fn handler(ctx: Context<CompleteTask>) -> Result<()> {
    let agent = &mut ctx.accounts.agent;
    agent.reputation += 1;
    Ok(())
}
