use crate::state::*;
use anchor_lang::prelude::*;

pub fn add_to_whitelist_handler(ctx: Context<AddToWhitelist>, buyer: Pubkey) -> Result<()> {
    let buyer_info = &mut ctx.accounts.buyer_info;
    buyer_info.buyer = buyer;
    buyer_info.is_whitelisted = true;
    Ok(())
}

#[derive(Accounts)]
#[instruction(buyer: Pubkey)]
pub struct AddToWhitelist<'info> {
    #[account(
        init,
        seeds = [b"buyer_info".as_ref(), buyer.key().as_ref()],
        bump,
        payer = admin,
        space = 8 + BuyerInfo::INIT_SPACE
    )]
    pub buyer_info: Account<'info, BuyerInfo>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
