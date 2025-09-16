use crate::{
    error::ErrorCode,
    state::{Multisig, Transaction, TransactionAccount},
};
use anchor_lang::prelude::*;

// Creates a new transaction account, automatically signed by the creator, which must be one of the owners of the multisig.
#[derive(Accounts)]
pub struct CreateTransaction<'info> {
    multisig: Box<Account<'info, Multisig>>,
    #[account(zero, signer)]
    transaction: Box<Account<'info, Transaction>>,
    // One of the owners. Checked in the handler.
    proposer: Signer<'info>,
}

pub fn create_transaction_handler(
    ctx: Context<CreateTransaction>,
    pid: Pubkey,
    accs: Vec<TransactionAccount>,
    data: Vec<u8>,
) -> Result<()> {
    let owner_index = ctx
        .accounts
        .multisig
        .owners
        .iter()
        .position(|a| a == ctx.accounts.proposer.key)
        .ok_or(ErrorCode::InvalidOwner)?;

    let mut signers = Vec::new();
    signers.resize(ctx.accounts.multisig.owners.len(), false);
    signers[owner_index] = true;

    let tx = &mut ctx.accounts.transaction;
    tx.program_id = pid;
    tx.accounts = accs;
    tx.data = data;
    tx.signers = signers;
    tx.multisig = ctx.accounts.multisig.key();
    tx.did_execute = false;
    tx.owner_set_seqno = ctx.accounts.multisig.owner_set_seqno;

    Ok(())
}
