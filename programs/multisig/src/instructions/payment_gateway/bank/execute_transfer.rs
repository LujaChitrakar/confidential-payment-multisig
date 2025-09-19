use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::{
    error::ErrorCode,
    state::{BankAccount, KycRecord, TransferRequest, TransferStatus},
    validate_compliance, validate_sanctions,
};

#[derive(Accounts)]
pub struct ExecuteTransfer<'info> {
    #[account(mut)]
    pub transfer_request: Account<'info, TransferRequest>,
    #[account(mut)]
    pub sender_bank: Account<'info, BankAccount>,
    pub sender_kyc: Account<'info, KycRecord>,
    pub recipient_kyc: Account<'info, KycRecord>,
    #[account(mut)]
    pub sender_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn execute_transfer_handler(ctx: Context<ExecuteTransfer>) -> Result<()> {
    let transfer_request = &mut ctx.accounts.transfer_request;

    // Check approval requirements
    if transfer_request.required_multisig_approval {
        require!(
            transfer_request.multisig_approved,
            ErrorCode::MultisigApprovalRequired
        );
        require!(
            transfer_request.status == TransferStatus::Approved,
            ErrorCode::TransferNotApproved
        );
    } else {
        require!(
            transfer_request.status == TransferStatus::Pending
                || transfer_request.status == TransferStatus::Approved,
            ErrorCode::TransferNotApproved
        );
    }

    // Perform compliance checks
    validate_compliance(&ctx.accounts.sender_kyc, &ctx.accounts.recipient_kyc)?;
    validate_sanctions(&ctx.accounts.sender_kyc, &ctx.accounts.recipient_kyc)?;

    // Execute token transfer
    let bank_seeds = &[
        b"bank",
        ctx.accounts.sender_bank.bank_id.as_bytes(),
        &[ctx.accounts.sender_bank.bump],
    ];
    let bank_signer = &[&bank_seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.sender_vault.to_account_info(),
        to: ctx.accounts.recipient_vault.to_account_info(),
        authority: ctx.accounts.sender_bank.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, bank_signer);

    transfer(cpi_ctx, transfer_request.amount)?;

    // Update status and records
    transfer_request.status = TransferStatus::Executed;
    transfer_request.executed_at = Some(Clock::get()?.unix_timestamp);

    // Update bank statistics
    let sender_bank = &mut ctx.accounts.sender_bank;
    sender_bank.total_transfers += 1;
    sender_bank.total_volume += transfer_request.amount;

    Ok(())
}
