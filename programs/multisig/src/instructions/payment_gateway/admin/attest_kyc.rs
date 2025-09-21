use crate::state::gateway::KycRecord;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(bank_id: u64)]
pub struct AttestKyc<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + KycRecord::INIT_SPACE,
        seeds = [b"kyc",bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub kyc_record: Account<'info, KycRecord>,
    pub system_program: Program<'info, System>,
}

pub fn attest_kyc_handler(ctx: Context<AttestKyc>, bank_id: u64) -> Result<()> {
    let kyc_record = &mut ctx.accounts.kyc_record;
    kyc_record.is_active = true;
    kyc_record.verified_by = ctx.accounts.authority.key();
    kyc_record.updated_at = Clock::get()?.unix_timestamp;
    kyc_record.bank_id = bank_id;

    Ok(())
}
