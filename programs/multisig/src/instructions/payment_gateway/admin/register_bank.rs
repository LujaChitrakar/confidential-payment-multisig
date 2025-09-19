use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{
    error::ErrorCode,
    state::{BankAccount, PaymentGateway},
};

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

    #[account()]
    pub usdc_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds=[b"multisig", bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub banks_multisig_signer: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=usdc_mint,
        associated_token::authority=banks_multisig_signer
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

pub fn register_bank_handler(
    ctx: Context<RegisterBank>,
    bank_id: u64,
    swift_code: String,
) -> Result<()> {
    // This function is called through multisig execute_transaction
    // The multisig_signer will be validated by the PDA seeds

    let bank = &mut ctx.accounts.bank;
    bank.bank_id = bank_id;
    bank.swift_code = swift_code;
    bank.is_active = true;
    // bank.bank_multisig = ctx.accounts.banks_multisig_signer.key();
    bank.treasury_ata = ctx.accounts.treasury_ata.key();
    bank.is_active = true;

    let gateway = &mut ctx.accounts.gateway;
    gateway.total_banks += 1;

    Ok(())
}
