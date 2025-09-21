use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::state::gateway::BankAccount;

#[derive(Accounts)]
#[instruction(bank_id:u64,recipient:Pubkey)]
pub struct BankWithdraw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account()]
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"bank", &bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankAccount>,

    // #[account(mut)]
    // pub banks_multisig_signer: Account<'info, Multisig>,
    #[account(
         mut,
        associated_token::mint=usdc_mint,
        associated_token::authority=recipient
    )]
    pub reciepient_ata: Account<'info, TokenAccount>,

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

pub fn withdraw_handler(ctx: Context<BankWithdraw>, amount: u64) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
    let bank_id_bytes = bank.bank_id.to_le_bytes();

    let signer_seeds: &[&[&[u8]]] = &[&[b"bank", &bank_id_bytes[..], &[ctx.bumps.bank]]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.treasury_ata.to_account_info(),
            to: ctx.accounts.reciepient_ata.to_account_info(),
            authority: bank.to_account_info(),
        },
        signer_seeds,
    );
    transfer(cpi_ctx, amount)?;

    bank.balance -= amount;
    Ok(())
}
