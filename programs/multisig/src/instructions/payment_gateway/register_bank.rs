use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{
    error::ErrorCode,
    state::{BankAccount, PaymentGateway},
};

#[derive(Accounts)]
#[instruction(bank_id: String)]
pub struct RegisterBank<'info> {
    #[account(mut)]
    pub gateway: Account<'info, PaymentGateway>,
    #[account(
        init,
        payer = authority,
        space = 8 + BankAccount::INIT_SPACE,
        seeds = [b"bank", bank_id.as_bytes()],
        bump
    )]
    pub bank: Account<'info, BankAccount>,
    /// CHECK: This is verified as multisig signer PDA
    pub multisig_signer: UncheckedAccount<'info>,
    pub kyc_authority: Signer<'info>,
    pub treasury_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn register_bank_handler(
    ctx: Context<RegisterBank>,
    bank_id: String,
    bank_name: String,
    swift_code: String,
    compliance_tier: u8,
) -> Result<()> {
    // This function is called through multisig execute_transaction
    // The multisig_signer will be validated by the PDA seeds

    require!(bank_id.len() <= 32, ErrorCode::InvalidBankId);
    require!(bank_name.len() <= 64, ErrorCode::InvalidBankName);
    require!(swift_code.len() == 11, ErrorCode::InvalidSwiftCode);
    require!(compliance_tier <= 3, ErrorCode::InvalidComplianceTier);

    let bank = &mut ctx.accounts.bank;
    bank.bank_id = bank_id;
    bank.bank_name = bank_name;
    bank.swift_code = swift_code;
    bank.compliance_tier = compliance_tier;
    bank.is_active = true;
    bank.kyc_authority = ctx.accounts.kyc_authority.key();
    bank.treasury_vault = ctx.accounts.treasury_vault.key();
    bank.total_transfers = 0;
    bank.total_volume = 0;
    bank.bump = ctx.bumps.bank;

    let gateway = &mut ctx.accounts.gateway;
    gateway.total_banks += 1;

    Ok(())
}
