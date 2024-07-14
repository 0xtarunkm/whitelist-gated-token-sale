use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum ErrorCode {
    #[msg("You are not whitelisted to participate in this sale.")]
    NotWhitelisted,
    #[msg("You have exceeded the limit per wallet.")]
    ExceedsLimit,
    #[msg("The sale does not have enough tokens.")]
    InsufficientTokens,
    #[msg("You do not have enough funds to buy tokens.")]
    InsufficientFunds,
}
