use anchor_lang::prelude::*;

#[error_code]
pub enum BankError {
    #[msg("Yatırılacak miktar sıfırdan büyük olmalıdır.")]
    InvalidAmount,

    #[msg("Bakiye sınırı aşıldı.")]
    Overflow,

    #[msg("Yetersiz bakiye.")]
InsufficientFunds,
}