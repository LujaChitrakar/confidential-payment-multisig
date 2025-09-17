use crate::{
    error::ErrorCode,
    state::{Multisig, Transaction},
};
use anchor_lang::prelude::*;

// Approves a transaction on behalf of an owner of the multisig.
#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(constraint = multisig.owner_set_seqno == transaction.owner_set_seqno)]
    multisig: Box<Account<'info, Multisig>>,
    #[account(mut, has_one = multisig)]
    transaction: Box<Account<'info, Transaction>>,
    // One of the multisig owners. Checked in the handler.
    owner: Signer<'info>,
}

// Approves a transaction on behalf of an owner of the multisig.
pub fn approve_handler(ctx: Context<Approve>) -> Result<()> {
    let owner_index = ctx
        .accounts
        .multisig
        .owners
        .iter()
        .position(|a| a == ctx.accounts.owner.key)
        .ok_or(ErrorCode::InvalidOwner)?;

    ctx.accounts.transaction.signers[owner_index] = true;

    Ok(())
}
