pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("FTvCUaTYSVNYUwLWk4yxgvW3Y81e1HjWD8nLxbxxKt5T");

#[program]
pub mod penne_bank_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        crate::instructions::deposit::handle_deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        crate::instructions::withdraw::handle_withdraw(ctx, amount)
    }
}