use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, TokenAccount, Transfer},
};

use crate::state::{BankAccount, UserStats};

#[derive(Accounts)]
#[instruction(bank_id:u64,recipient:Pubkey)]
pub struct EmergencyFreeze<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bank", &bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankAccount>,

    #[account(
        mut,
        seeds=[b"multisig", bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub banks_multisig_signer: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

pub fn emergency_freeze_handler(ctx: Context<EmergencyFreeze>) -> Result<()> {
    let bank = &mut ctx.accounts.bank;

    bank.is_active = false;
    Ok(())
}
