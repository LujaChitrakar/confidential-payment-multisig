use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::state::{BankAccount, UserStats};

#[derive(Accounts)]
#[instruction(bank_id:u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account()]
    pub usdc_mint:Account<'info,Mint>,

    #[account(
        mut,
        seeds = [b"bank", &bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankAccount>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"deposit", user.key().as_ref()],
        space = UserStats::INIT_SPACE,
        bump,
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(
         mut,
        associated_token::mint=usdc_mint,
        associated_token::authority=user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=usdc_mint,
        associated_token::authority=bank.bank_admin
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let user_stats = &mut ctx.accounts.user_stats;
    let bank = &mut ctx.accounts.bank;

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.user_ata.to_account_info(),
            to: ctx.accounts.treasury_ata.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;

    user_stats.balance += amount;
    bank.balance += amount;
    Ok(())
}
