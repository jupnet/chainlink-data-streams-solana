use crate::state::VerifierAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VerifyContext<'info> {
    #[account(seeds = [b"verifier"], bump)]
    pub verifier_account: AccountLoader<'info, VerifierAccount>,
    pub user: Signer<'info>,
    /// CHECK: Program will validate this based on report input.
    pub config_account: UncheckedAccount<'info>
}
