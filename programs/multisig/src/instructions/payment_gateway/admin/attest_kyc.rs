use anchor_lang::prelude::*;

use crate::state::KycRecord;

#[derive(Accounts)]
#[instruction(user_pubkey:Pubkey)]
pub struct AttestKyc<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + KycRecord::INIT_SPACE,
        seeds = [b"kyc",user_pubkey.as_ref()],
        bump
    )]
    pub kyc_record: Account<'info, KycRecord>,
    pub system_program: Program<'info, System>,
}

pub fn attest_kyc_handler(ctx: Context<AttestKyc>, user_pubkey: Pubkey) -> Result<()> {
    let kyc_record = &mut ctx.accounts.kyc_record;
    kyc_record.is_active = true;
    kyc_record.verified_by = ctx.accounts.authority.key();
    kyc_record.updated_at = Clock::get()?.unix_timestamp;
    kyc_record.owner = user_pubkey;

    Ok(())
}
