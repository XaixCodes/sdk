use anchor_lang::prelude::*;

#[account]
pub struct Agent {
    pub authority: Pubkey,
    pub reputation: u64,
}

#[account]
pub struct Task {
    pub creator: Pubkey,
    pub completed: bool,
}
