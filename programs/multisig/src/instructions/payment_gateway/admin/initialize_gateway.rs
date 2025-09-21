use anchor_lang::prelude::*;

use crate::state::gateway::PaymentGateway;

#[derive(Accounts)]
pub struct InitializeGateway<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        seeds=[b"gateway"],
        space = 8 + PaymentGateway::INIT_SPACE,
        bump
    )]
    pub gateway: Account<'info, PaymentGateway>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_gateway_handler(
    ctx: Context<InitializeGateway>,
    admin: Pubkey,
    // treasury: Pubkey,
    // fee_bps: u16,
) -> Result<()> {
    let gateway = &mut ctx.accounts.gateway;
    gateway.admin = admin;
    gateway.total_banks = 0;
    // gateway.treasury = treasury;
    // gateway.fee_bps = fee_bps;
    gateway.is_active = true;

    Ok(())
}
