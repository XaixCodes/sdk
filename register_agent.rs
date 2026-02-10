use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct RegisterAgent<'info> {
    #[account(init, payer = signer, space = 8 + 32 + 8)]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterAgent>) -> Result<()> {
    let agent = &mut ctx.accounts.agent;
    agent.authority = ctx.accounts.signer.key();
    agent.reputation = 0;
    Ok(())
}
