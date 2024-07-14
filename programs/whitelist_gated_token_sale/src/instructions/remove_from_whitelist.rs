use crate::state::*;
use anchor_lang::prelude::*;

pub fn remove_from_whitelist_handler(
    ctx: Context<RemoveFromWhitelist>,
    buyer: Pubkey,
) -> Result<()> {
    let buyer_info = &mut ctx.accounts.buyer_info;
    buyer_info.buyer = buyer;
    buyer_info.is_whitelisted = false;
    Ok(())
}

#[derive(Accounts)]
#[instruction(buyer: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(
        mut,
        seeds = [b"buyer_info".as_ref(), buyer.key().as_ref()],
        bump,
        realloc = 8 + BuyerInfo::INIT_SPACE,
        realloc::payer = admin,
        realloc::zero = true,
    )]
    pub buyer_info: Account<'info, BuyerInfo>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}
