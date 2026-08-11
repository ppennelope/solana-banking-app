use anchor_lang::prelude::*;

use crate::{error::BankError, BankAccount};

pub fn handle_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, BankError::InvalidAmount);

    let bank_account = &mut ctx.accounts.bank_account;

    bank_account.balance = bank_account
        .balance
        .checked_add(amount)
        .ok_or(BankError::Overflow)?;

    msg!("{} birim yatırıldı.", amount);
    msg!("Yeni bakiye: {}", bank_account.balance);

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"bank-account", owner.key().as_ref()],
        bump = bank_account.bump,
        has_one = owner
    )]
    pub bank_account: Account<'info, BankAccount>,

    pub owner: Signer<'info>,
}