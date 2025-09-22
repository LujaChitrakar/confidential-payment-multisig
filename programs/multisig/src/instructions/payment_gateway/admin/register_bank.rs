use crate::{
    error::ErrorCode,
    state::gateway::{BankAccount, KycRecord, PaymentGateway},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
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
        mut,
        seeds = [b"kyc",bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub kyc_record: Account<'info, KycRecord>,

    #[account(
        init,
        payer = authority,
        space = 8 + BankAccount::INIT_SPACE,
        seeds = [b"bank", bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankAccount>,

    pub system_program: Program<'info, System>,
}

pub fn register_bank_handler(
    ctx: Context<RegisterBank>,
    bank_id: u64,
    bank_name: String,
    swift_code: String,
) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
    require!(ctx.accounts.kyc_record.is_active, ErrorCode::KycNotVerified);
    bank.bank_id = bank_id;
    bank.bank_name = bank_name;
    bank.swift_code = swift_code;
    bank.is_active = true;

    let gateway = &mut ctx.accounts.gateway;
    gateway.total_banks += 1;

    Ok(())
}
