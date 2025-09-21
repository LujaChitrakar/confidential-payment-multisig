use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::gateway::{BankAccount, PaymentGateway};

#[derive(Accounts)]
#[instruction(bank_id: u64)]
pub struct RegisterBank<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds=[b"gateway"],
        bump,
    )]
    pub gateway: Account<'info, PaymentGateway>,

    #[account(
        init,
        payer = authority,
        space = 8 + BankAccount::INIT_SPACE,
        seeds = [b"bank", bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankAccount>,

    // #[account()]
    pub usdc_mint: Account<'info, Mint>,

    // #[account(mut)]
    // pub banks_multisig_signer: Account<'info, Multisig>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint=usdc_mint,
        associated_token::authority=bank
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn register_bank_handler(
    ctx: Context<RegisterBank>,
    bank_id: u64,
    bank_name: String,
    swift_code: String,
) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
    bank.bank_id = bank_id;
    bank.bank_name = bank_name;
    bank.swift_code = swift_code;
    // bank.bank_multisig = ctx.accounts.banks_multisig_signer.key();
    bank.treasury_ata = ctx.accounts.treasury_ata.key();
    bank.is_active = true;

    let gateway = &mut ctx.accounts.gateway;
    gateway.total_banks += 1;

    Ok(())
}
