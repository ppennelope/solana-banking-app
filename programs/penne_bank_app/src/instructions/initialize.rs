use anchor_lang::prelude::*;

use crate::BankAccount;

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    let bank_account = &mut ctx.accounts.bank_account;

    bank_account.owner = ctx.accounts.user.key();
    bank_account.balance = 0;
    bank_account.bump = ctx.bumps.bank_account;

    msg!("Banka hesabı oluşturuldu.");
    msg!("Başlangıç bakiyesi: 0");

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + BankAccount::INIT_SPACE,
        seeds = [b"bank-account", user.key().as_ref()],
        bump
    )]
    pub bank_account: Account<'info, BankAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}