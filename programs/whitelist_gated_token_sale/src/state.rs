use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Sale {
    #[max_len(200)]
    pub token_name: String,
    pub sale_amount: u64,
    pub price_per_token: u64,
    pub sold_amount: u64,
}

#[account]
#[derive(InitSpace)]
pub struct BuyerInfo {
    pub buyer: Pubkey,
    pub is_whitelisted: bool,
    pub purchased_amount: u64,
}
