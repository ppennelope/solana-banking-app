use anchor_lang::prelude::*;

use crate::{error::BankError, BankAccount};

pub fn handle_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, BankError::InvalidAmount);

    let bank_account = &mut ctx.accounts.bank_account;

    require!(
        bank_account.balance >= amount,
        BankError::InsufficientFunds
    );

    bank_account.balance = bank_account
        .balance
        .checked_sub(amount)
        .ok_or(BankError::Overflow)?;

    msg!("{} birim çekildi.", amount);
    msg!("Yeni bakiye: {}", bank_account.balance);

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"bank-account", owner.key().as_ref()],
        bump = bank_account.bump,
        has_one = owner
    )]
    pub bank_account: Account<'info, BankAccount>,

    pub owner: Signer<'info>,
}