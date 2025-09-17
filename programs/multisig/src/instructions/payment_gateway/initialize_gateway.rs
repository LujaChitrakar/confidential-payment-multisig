use anchor_lang::prelude::*;

use crate::state::PaymentGateway;

#[derive(Accounts)]
pub struct InitializeGateway<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + PaymentGateway::INIT_SPACE
    )]
    pub gateway: Account<'info, PaymentGateway>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_gateway_handler(
    ctx: Context<InitializeGateway>,
    admin_multisig: Pubkey,
    compliance_multisig: Pubkey,
) -> Result<()> {
    let gateway = &mut ctx.accounts.gateway;
    gateway.admin_multisig = admin_multisig;
    gateway.compliance_multisig = compliance_multisig;
    gateway.is_active = true;
    gateway.total_banks = 0;
    gateway.large_transfer_threshold = 100_000; // Default $100k
    gateway.max_daily_volume = 10_000_000; // Default $10M
    Ok(())
}
