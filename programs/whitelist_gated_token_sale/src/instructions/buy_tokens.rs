use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};

pub fn buy_tokens_handler(ctx: Context<BuyTokens>, _token_name: String, amount: u64) -> Result<()> {
    let sale = &mut ctx.accounts.sale;
    let buyer = &mut ctx.accounts.buyer;
    let buyer_info = &mut ctx.accounts.buyer_info;

    require!(buyer_info.is_whitelisted, ErrorCode::NotWhitelisted);
    require!(
        buyer_info.purchased_amount + amount <= sale.sale_amount,
        ErrorCode::ExceedsLimit
    );
    require!(
        sale.sold_amount + amount <= sale.sale_amount,
        ErrorCode::InsufficientTokens
    );

    let cost = sale.price_per_token * amount;
    require!(
        buyer.to_account_info().lamports() >= cost,
        ErrorCode::InsufficientFunds
    );

    sale.sold_amount += amount;
    buyer_info.purchased_amount += amount;

    let transfer_ix = transfer(&ctx.accounts.buyer.key(), &ctx.accounts.sale.key(), amount);

    invoke(
        &transfer_ix,
        &[
            ctx.accounts.buyer.to_account_info(),
            ctx.accounts.sale.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(token_name: String)]
pub struct BuyTokens<'info> {
    #[account(
        mut,
        seeds = [b"sale".as_ref(), token_name.as_bytes()],
        bump,
        realloc = 8 + Sale::INIT_SPACE,
        realloc::payer = buyer,
        realloc::zero = true,
    )]
    pub sale: Account<'info, Sale>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"buyer_info".as_ref(), buyer.key().as_ref()],
        bump,
        realloc = 8 + BuyerInfo::INIT_SPACE,
        realloc::payer = buyer,
        realloc::zero = true,
        has_one = buyer
    )]
    pub buyer_info: Account<'info, BuyerInfo>,
    pub system_program: Program<'info, System>,
}
