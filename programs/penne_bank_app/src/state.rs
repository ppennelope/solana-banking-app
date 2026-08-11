use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BankAccount {
    pub owner: Pubkey,
    pub balance: u64,
    pub bump: u8,
}