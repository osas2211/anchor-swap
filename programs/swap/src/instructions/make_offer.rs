use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{state::offer::*, transfer_token, ANCHOR_DISCRIMINATOR};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
    mut,
    associated_token::mint = token_mint_a,
    associated_token::authority = maker,
    associated_token::token_program = token_program
  )]
    pub maker_a_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,

    #[account(
    init,
    payer = maker,
    space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
    seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
    bump
  )]
    pub offer: Account<'info, Offer>,
    pub system_program: Program<'info, System>,
    #[account(
    mut,
    associated_token::mint = token_mint_a,
    associated_token::authority = offer,
    associated_token::token_program = token_program
  )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn send_offered_tokens_to_vault(
    context: &Context<MakeOffer>,
    token_a_offered_offered: u64,
) -> Result<()> {
    transfer_token(
        &context.accounts.maker_a_token_account,
        &context.accounts.vault,
        &token_a_offered_offered,
        &context.accounts.token_mint_a,
        &context.accounts.maker,
        &context.accounts.token_program,
    )
}

pub fn save_offer(ctx: Context<MakeOffer>, id: u64, token_b_wanted_amount: u64) -> Result<()> {
    ctx.accounts.offer.set_inner(Offer {
        id,
        maker: ctx.accounts.maker.key(),
        token_mint_a: ctx.accounts.token_mint_a.key(),
        token_mint_b: ctx.accounts.token_mint_b.key(),
        token_b_wanted_amount,
        bump: ctx.bumps.offer,
    });
    Ok(())
}
