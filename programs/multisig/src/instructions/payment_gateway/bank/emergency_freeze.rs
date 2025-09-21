use anchor_lang::prelude::*;

use crate::{error::ErrorCode, state::gateway::BankAccount};

#[derive(Accounts)]
#[instruction(bank_id:u64)]
pub struct EmergencyFreeze<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bank", &bank_id.to_le_bytes().as_ref()],
        bump
    )]
    pub bank: Account<'info, BankAccount>,

    pub system_program: Program<'info, System>,
}

pub fn emergency_freeze_handler(ctx: Context<EmergencyFreeze>, bank_id: u64) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
    require!(bank.bank_id == bank_id, ErrorCode::InvalidBankId);

    bank.is_active = false;
    Ok(())
}
