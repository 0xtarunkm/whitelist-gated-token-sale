use anchor_lang::prelude::*;
use instructions::add_to_whitelist::*;
use instructions::buy_tokens::*;
use instructions::init::*;
use instructions::remove_from_whitelist::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("GkvBG6cxfEBnuSvb2Zk5tA2qb3aPWWL3doj43dUKHf23");

#[program]
pub mod whitelist_gated_token_sale {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        token_name: String,
        sale_amount: u64,
        price_per_token: u64,
    ) -> Result<()> {
        initialize_handler(ctx, token_name, sale_amount, price_per_token)
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, token_name: String, amount: u64) -> Result<()> {
        buy_tokens_handler(ctx, token_name, amount)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, buyer: Pubkey) -> Result<()> {
        add_to_whitelist_handler(ctx, buyer)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, buyer: Pubkey) -> Result<()> {
        remove_from_whitelist_handler(ctx, buyer)
    }
}
