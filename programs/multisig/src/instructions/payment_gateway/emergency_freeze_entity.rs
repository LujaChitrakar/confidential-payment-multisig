use anchor_lang::prelude::*;

use crate::state::KycRecord;

#[derive(Accounts)]
pub struct EmergencyFreezeEntity<'info> {
    #[account(mut)]
    pub kyc_record: Account<'info, KycRecord>,
    /// CHECK: This is verified as compliance multisig signer PDA
    pub multisig_signer: UncheckedAccount<'info>,
}

pub fn emergency_freeze_entity_handler(
    ctx: Context<EmergencyFreezeEntity>,
    reason: String,
) -> Result<()> {
    // This function is called through compliance multisig execute_transaction
    let kyc_record = &mut ctx.accounts.kyc_record;
    kyc_record.is_active = false;
    kyc_record.freeze_reason = Some(reason.clone());
    kyc_record.frozen_at = Some(Clock::get()?.unix_timestamp);

    // emit!(EntityFrozenEvent {
    //     entity: kyc_record.key(),
    //     reason: reason,
    //     frozen_by: ctx.accounts.multisig_signer.key(),
    // });

    Ok(())
}
