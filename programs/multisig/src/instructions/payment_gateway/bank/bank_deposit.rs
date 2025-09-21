use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{spl_token::state::Multisig, transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::state::gateway::BankAccount;

#[derive(Accounts)]
#[instruction(bank_id:u64,recipient:Pubkey)]
pub struct BankDeposit<'info> {
    #[account(mut)]
    pub deposit_authority: Signer<'info>,

    // #[account()]
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"bank", &bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankAccount>,

    #[account(
        mut,
        associated_token::mint=usdc_mint,
        associated_token::authority=deposit_authority
    )]
    pub deposit_authority_ata: Account<'info, TokenAccount>,

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

pub fn deposit_handler(ctx: Context<BankDeposit>, amount: u64) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.deposit_authority_ata.to_account_info(),
            to: ctx.accounts.treasury_ata.to_account_info(),
            authority: ctx.accounts.deposit_authority.to_account_info(),
        },
    );
    transfer(cpi_ctx, amount)?;

    bank.balance -= amount;
    Ok(())
}
