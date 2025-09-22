use crate::{
    error::ErrorCode,
    state::{
        multisig::MultiSig,
        transaction::{Transaction, TransactionStatus},
    },
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(owner_stratum: u8)]
pub struct AcceptTransaction<'info> {
    #[account(
        mut,
        seeds = [
            b"transaction",
            multi_sig.key().as_ref(),
            &transaction.transaction_num.to_le_bytes()
        ],
        bump = transaction.bump,
        constraint = transaction.multi_sig == multi_sig.key() @ ErrorCode::InvalidTransaction,
        constraint = transaction.status == TransactionStatus::Vote @ ErrorCode::NotVoteStatus,
        constraint = transaction.version == multi_sig.version @ ErrorCode::VersionOutdated
    )]
    pub transaction: Account<'info, Transaction>,

    #[account(
        seeds = [
            b"multi_sig",
            multi_sig.creator.as_ref(),
            multi_sig.name.as_bytes()
        ],
        bump = multi_sig.multisig_bump,
        constraint = multi_sig.strata.len() > owner_stratum as usize @ ErrorCode::InvalidStratumNumber
    )]
    pub multi_sig: Account<'info, MultiSig>,

    #[account(
        constraint = multi_sig.is_owner_stratum(signer.key(), owner_stratum as usize).is_some() @ ErrorCode::InvalidOwner
    )]
    pub signer: Signer<'info>,
}

pub fn accept_transaction_handler(
    ctx: Context<AcceptTransaction>,
    _owner_stratum: u8,
) -> Result<()> {
    let transaction = &mut ctx.accounts.transaction;
    let multi_sig = &ctx.accounts.multi_sig;

    for (index, stratum) in multi_sig.strata.iter().enumerate() {
        if stratum.active {
            require_gte!(
                transaction.accepted[index],
                stratum.m,
                ErrorCode::InsufficientVotes
            );
        }
    }

    transaction.status = TransactionStatus::Accepted;
    Ok(())
}
