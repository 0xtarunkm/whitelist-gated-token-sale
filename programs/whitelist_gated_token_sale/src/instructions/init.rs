use crate::state::*;
use anchor_lang::prelude::*;

pub fn initialize_handler(
    ctx: Context<Initialize>,
    token_name: String,
    sale_amount: u64,
    price_per_token: u64,
) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    sale.token_name = token_name;
    sale.sale_amount = sale_amount;
    sale.price_per_token = price_per_token;
    sale.sold_amount = 0;
    Ok(())
}

#[derive(Accounts)]
#[instruction(token_name: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"sale".as_ref(), token_name.as_bytes()],
        bump,
        payer = user,
        space = 8 + Sale::INIT_SPACE
    )]
    pub sale: Account<'info, Sale>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
