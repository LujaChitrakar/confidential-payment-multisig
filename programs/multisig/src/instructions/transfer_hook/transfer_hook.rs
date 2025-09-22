// use crate::error::ErrorCode;
// use crate::state::gateway::{BankAccount, KycRecord};
// use anchor_lang::prelude::*;
// use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

// #[derive(Accounts)]
// pub struct TransferHook<'info> {
//     /// Sender token account
//     #[account(mut)]
//     pub source_token: Account<'info, TokenAccount>,

//     /// Receiver token account
//     #[account(mut)]
//     pub destination_token: Account<'info, TokenAccount>,

//     pub mint: Account<'info, Mint>,

//     #[account(
//         seeds = [b"kyc", sender_kyc.bank_id.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub sender_kyc: Account<'info, KycRecord>,

//     #[account(
//         seeds = [b"kyc", receiver_kyc.bank_id.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub receiver_kyc: Account<'info, KycRecord>,

//     #[account(
//         seeds = [b"bank", bank.bank_id.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub bank: Account<'info, BankAccount>,

//     pub token_program: Program<'info, TokenInterface>,
// }

// /// Called automatically on every Token-2022 transfer
// pub fn transfer_hook(ctx: Context<TransferHook>) -> Result<()> {
//     require!(ctx.accounts.sender_kyc.is_active, ErrorCode::KycNotVerified);
//     require!(
//         ctx.accounts.receiver_kyc.is_active,
//         ErrorCode::KycNotVerified
//     );

//     msg!("Transfer hook passed: sender & receiver KYC verified");
//     Ok(())
// }