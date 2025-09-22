use crate::{constants::USDC_MINT, error::ErrorCode, state::gateway::BankAccount};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(bank_id:u64)]
pub struct BankDeposit<'info> {
    #[account(
        mut,
        address=bank.bank_admin @ErrorCode::InvalidAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bank", bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankAccount>,

    #[account(address=USDC_MINT)]
    pub usdc_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint=usdc_mint,
        associated_token::authority=admin
    )]
    pub admin_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=usdc_mint,
        associated_token::authority=bank
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn bank_deposit_handler(ctx: Context<BankDeposit>, bank_id: u64, amount: u64) -> Result<()> {
    let bank = &mut ctx.accounts.bank;

    require!(bank.bank_id == bank_id, ErrorCode::InvalidBankId);
    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(bank.is_active, ErrorCode::InactiveStratum);

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.admin_ata.to_account_info(),
            to: ctx.accounts.treasury_ata.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;

    bank.balance += amount;
    Ok(())
}
