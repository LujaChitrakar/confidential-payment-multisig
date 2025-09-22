use crate::{
    constants::{USDC_MINT, WSOL_MINT},
    error::ErrorCode,
    jupiter::{program::Jupiter, types::RoutePlanStep},
    state::gateway::BankAccount,
};
use anchor_lang::{
    prelude::*,
    solana_program::{entrypoint::ProgramResult, instruction::Instruction, program::invoke_signed},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct SwapArgs {
    id: u8,
    route_plan: Vec<RoutePlanStep>,
    in_amount: u64,
    qouted_out_amount: u64,
    slippage_bps: u16,
    platform_fee_bps: u8,
}
#[derive(Accounts)]
#[instruction(bank_id:u64)]
pub struct Swap<'info> {
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
        associated_token::mint=quote_mint,
        associated_token::authority=bank
    )]
    pub authority_sol_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint=usdc_mint,
        associated_token::authority=bank
    )]
    pub authority_usdc_ata: Box<Account<'info, TokenAccount>>,

    #[account(address=USDC_MINT)]
    pub usdc_mint: Box<Account<'info, Mint>>,
    #[account(address=WSOL_MINT)]
    pub quote_mint: Box<Account<'info, Mint>>,

    pub jupiter_program: Program<'info, Jupiter>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn swap_handler(ctx: Context<Swap>, bank_id: u64, data: Vec<u8>) -> Result<()> {
    let bank = &mut ctx.accounts.bank;

    require!(bank.bank_id == bank_id, ErrorCode::InvalidBankId);
    require!(bank.is_active, ErrorCode::InactiveStratum);

    let pre_usdc_balance = ctx.accounts.authority_usdc_ata.amount;
    let pre_sol_balance = ctx.accounts.authority_sol_ata.amount;

    require!(bank.bank_id == bank_id, ErrorCode::InvalidBankId);

    let bank_id_bytes = bank.bank_id.to_le_bytes();
    let signer_seeds: &[&[&[u8]]] = &[&[b"bank", &bank_id_bytes[..], &[ctx.bumps.bank]]];

    swap_on_jupiter(
        ctx.remaining_accounts,
        ctx.accounts.jupiter_program.clone(),
        &data,
        &ctx.accounts.admin.key(),
        signer_seeds,
    )?;

    ctx.accounts.authority_usdc_ata.reload()?;
    ctx.accounts.authority_sol_ata.reload()?;

    let post_usdc_balance = ctx.accounts.authority_usdc_ata.amount;
    let post_sol_balance = ctx.accounts.authority_sol_ata.amount;

    let swap_to_usdc_args = SwapArgs::try_from_slice(&data[8..])?;
    let slippage_bps = swap_to_usdc_args.slippage_bps;
    let min_out_amount = ((swap_to_usdc_args.qouted_out_amount as u128)
        * (10000 - slippage_bps as u128)
        / 10000) as u64;

    //SWAP from LP -> USDC
    require!(post_usdc_balance > pre_usdc_balance, ErrorCode::InvalidSwap);

    let usdc_received = post_usdc_balance - pre_usdc_balance;
    let lp_tokens_sold = pre_sol_balance - post_sol_balance;

    require!(
        usdc_received >= min_out_amount,
        ErrorCode::InsufficientQuoteTokens
    );

    Ok(())
}

fn swap_on_jupiter<'info>(
    remaining_accounts: &[AccountInfo],
    jupiter_program: Program<'info, Jupiter>,
    data: &Vec<u8>,
    pda_signer: &Pubkey,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let accounts: Vec<AccountMeta> = remaining_accounts
        .iter()
        .map(|acc| {
            let is_signer = acc.key == pda_signer;

            return AccountMeta {
                pubkey: *acc.key,
                is_signer,
                is_writable: acc.is_writable,
            };
        })
        .collect();

    let accounts_infos: Vec<AccountInfo> = remaining_accounts
        .iter()
        .map(|acc| AccountInfo { ..acc.clone() })
        .collect();

    invoke_signed(
        &Instruction {
            program_id: *jupiter_program.key,
            accounts,
            data: data.to_vec(),
        },
        &accounts_infos,
        signer_seeds,
    )
}
