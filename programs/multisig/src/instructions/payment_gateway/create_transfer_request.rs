use anchor_lang::prelude::*;

use crate::{
    error::ErrorCode,
    state::{BankAccount, KycRecord, PaymentGateway, TransferRequest, TransferStatus},
};

#[derive(Accounts)]
#[instruction(transfer_id: String)]
pub struct CreateTransferRequest<'info> {
    pub gateway: Account<'info, PaymentGateway>,
    pub sender_bank: Account<'info, BankAccount>,
    pub sender_kyc: Account<'info, KycRecord>,
    pub recipient_kyc: Account<'info, KycRecord>,
    #[account(
        init,
        payer = authority,
        space = 8 + TransferRequest::INIT_SPACE,
        seeds = [b"transfer", transfer_id.as_bytes()],
        bump
    )]
    pub transfer_request: Account<'info, TransferRequest>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_transfer_request_handler(
    ctx: Context<CreateTransferRequest>,
    transfer_id: String,
    recipient_bank: Pubkey,
    amount: u64,
    currency: String,
    reference: String,
    compliance_metadata: String,
) -> Result<()> {
    require!(transfer_id.len() <= 64, ErrorCode::InvalidTransferId);
    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(currency.len() <= 8, ErrorCode::InvalidCurrency);

    let transfer_request = &mut ctx.accounts.transfer_request;
    transfer_request.transfer_id = transfer_id;
    transfer_request.sender_bank = ctx.accounts.sender_bank.key();
    transfer_request.recipient_bank = recipient_bank;
    transfer_request.sender_entity = ctx.accounts.sender_kyc.key();
    transfer_request.recipient_entity = ctx.accounts.recipient_kyc.key();
    transfer_request.amount = amount;
    transfer_request.currency = currency;
    transfer_request.reference = reference;
    transfer_request.compliance_metadata = compliance_metadata;
    transfer_request.status = TransferStatus::Pending;
    transfer_request.created_at = Clock::get()?.unix_timestamp;
    transfer_request.executed_at = None;

    // Check if multisig approval is required
    transfer_request.required_multisig_approval =
        amount >= ctx.accounts.gateway.large_transfer_threshold;
    transfer_request.multisig_approved = false;

    Ok(())
}
