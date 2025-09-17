use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode,
    state::{BankAccount, EntityType, KycRecord},
};

#[derive(Accounts)]
#[instruction(entity_id: String)]
pub struct AddKycEntity<'info> {
    pub bank: Account<'info, BankAccount>,
    #[account(
        init,
        payer = authority,
        space = 8 + KycRecord::INIT_SPACE,
        seeds = [b"kyc", entity_id.as_bytes(), bank.key().as_ref()],
        bump
    )]
    pub kyc_record: Account<'info, KycRecord>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn add_kyc_entity_handler(
    ctx: Context<AddKycEntity>,
    entity_id: String,
    entity_type: EntityType,
    compliance_score: u8,
    country_code: String,
) -> Result<()> {
    require!(entity_id.len() <= 64, ErrorCode::InvalidEntityId);
    require!(compliance_score <= 100, ErrorCode::InvalidComplianceScore);
    require!(country_code.len() == 2, ErrorCode::InvalidCountryCode);

    let kyc_record = &mut ctx.accounts.kyc_record;
    kyc_record.entity_id = entity_id;
    // kyc_record.entity_type = entity_type;
    kyc_record.compliance_score = compliance_score;
    kyc_record.country_code = country_code;
    kyc_record.is_active = true;
    kyc_record.verified_by = ctx.accounts.bank.key();
    kyc_record.verification_timestamp = Clock::get()?.unix_timestamp;
    kyc_record.freeze_reason = None;
    kyc_record.frozen_at = None;

    Ok(())
}
