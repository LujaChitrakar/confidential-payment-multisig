use crate::{
    error::ErrorCode,
    state::{Multisig, Transaction},
};
use anchor_lang::{
    prelude::*,
    solana_program::{self, instruction::Instruction},
};
use std::ops::Deref;

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(constraint = multisig.owner_set_seqno == transaction.owner_set_seqno)]
    multisig: Box<Account<'info, Multisig>>,
    /// CHECK: multisig_signer is a PDA program signer. Data is never read or written to
    #[account(
        seeds = [multisig.key().as_ref()],
        bump = multisig.nonce,
    )]
    multisig_signer: UncheckedAccount<'info>,
    #[account(mut, has_one = multisig)]
    transaction: Box<Account<'info, Transaction>>,
}

// Executes the given transaction if threshold owners have signed it.
pub fn execute_transaction_handler(ctx: Context<ExecuteTransaction>) -> Result<()> {
    // Has this been executed already?
    if ctx.accounts.transaction.did_execute {
        return Err(ErrorCode::AlreadyExecuted.into());
    }

    // Do we have enough signers.
    let sig_count = ctx
        .accounts
        .transaction
        .signers
        .iter()
        .filter(|&did_sign| *did_sign)
        .count() as u64;
    if sig_count < ctx.accounts.multisig.threshold {
        return Err(ErrorCode::NotEnoughSigners.into());
    }

    // Execute the transaction signed by the multisig.
    let mut ix: Instruction = (*ctx.accounts.transaction).deref().into();
    ix.accounts = ix
        .accounts
        .iter()
        .map(|acc| {
            let mut acc = acc.clone();
            if &acc.pubkey == ctx.accounts.multisig_signer.key {
                acc.is_signer = true;
            }
            acc
        })
        .collect();
    let multisig_key = ctx.accounts.multisig.key();
    let seeds = &[multisig_key.as_ref(), &[ctx.accounts.multisig.nonce]];
    let signer = &[&seeds[..]];
    let accounts = ctx.remaining_accounts;
    solana_program::program::invoke_signed(&ix, accounts, signer)?;

    // Burn the transaction to ensure one time use.
    ctx.accounts.transaction.did_execute = true;

    Ok(())
}
