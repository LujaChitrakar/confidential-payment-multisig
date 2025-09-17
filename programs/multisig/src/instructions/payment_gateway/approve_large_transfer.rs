use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode,
    state::{TransferRequest, TransferStatus},
};

#[derive(Accounts)]
pub struct ApproveLargeTransfer<'info> {
    #[account(mut)]
    pub transfer_request: Account<'info, TransferRequest>,
    /// CHECK: This is verified as compliance multisig signer PDA
    pub multisig_signer: UncheckedAccount<'info>,
}

pub fn approve_large_transfer_handler(ctx: Context<ApproveLargeTransfer>) -> Result<()> {
    // This function is called through compliance multisig execute_transaction
    let transfer_request = &mut ctx.accounts.transfer_request;

    require!(
        transfer_request.required_multisig_approval,
        ErrorCode::ApprovalNotRequired
    );
    require!(
        transfer_request.status == TransferStatus::Pending,
        ErrorCode::InvalidTransferStatus
    );

    transfer_request.multisig_approved = true;
    transfer_request.status = TransferStatus::Approved;

    Ok(())
}
