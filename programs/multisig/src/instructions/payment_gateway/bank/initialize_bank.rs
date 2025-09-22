use crate::{error::ErrorCode, state::gateway::BankAccount};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(bank_id:u64)]
pub struct InitializeBank<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bank", bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankAccount>,

    #[account(
        init,
        payer = admin,
        associated_token::mint=usdc_mint,
        associated_token::authority=bank
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_bank_handler(
    ctx: Context<InitializeBank>,
    bank_id: u64,
    instant_withdrawl_limit: u64,
) -> Result<()> {
    let bank = &mut ctx.accounts.bank;

    require!(bank.bank_id == bank_id, ErrorCode::InvalidBankId);
    require!(bank.is_active, ErrorCode::InactiveStratum);

    bank.bank_admin = ctx.accounts.admin.key();
    bank.instant_withdrawl_limit = instant_withdrawl_limit;
    bank.balance = 0;
    bank.treasury_ata = ctx.accounts.treasury_ata.key();
    bank.is_active = true;

    Ok(())
}
