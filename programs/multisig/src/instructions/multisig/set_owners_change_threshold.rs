use crate::{assert_unique_owners, error::ErrorCode, state::Multisig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Auth<'info> {
    #[account(mut)]
    multisig: Box<Account<'info, Multisig>>,
    #[account(
        seeds = [multisig.key().as_ref()],
        bump = multisig.nonce,
    )]
    multisig_signer: Signer<'info>,
}

pub fn set_owners_and_change_threshold_handler<'info>(
    ctx: Context<'_, '_, '_, 'info, Auth<'info>>,
    owners: Vec<Pubkey>,
    threshold: u64,
) -> Result<()> {
    require!(!owners.is_empty(), ErrorCode::InvalidOwnersLen);
    require!(threshold > 0, ErrorCode::InvalidThreshold);

    let multisig = &mut ctx.accounts.multisig;

    if threshold > multisig.owners.len() as u64 {
        return Err(ErrorCode::InvalidThreshold.into());
    }
    if (owners.len() as u64) < multisig.threshold {
        multisig.threshold = owners.len() as u64;
    }

    multisig.owners = owners;
    multisig.owner_set_seqno += 1;

    multisig.threshold = threshold;
    Ok(())
}

// Sets the owners field on the multisig. The only way this can be invoked
// is via a recursive call from execute_transaction -> set_owners.
pub fn set_owners_handler(ctx: Context<Auth>, owners: Vec<Pubkey>) -> Result<()> {
    assert_unique_owners(&owners)?;
    require!(!owners.is_empty(), ErrorCode::InvalidOwnersLen);

    let multisig = &mut ctx.accounts.multisig;

    if (owners.len() as u64) < multisig.threshold {
        multisig.threshold = owners.len() as u64;
    }

    multisig.owners = owners;
    multisig.owner_set_seqno += 1;

    Ok(())
}

// Changes the execution threshold of the multisig. The only way this can be
// invoked is via a recursive call from execute_transaction ->
// change_threshold.
pub fn change_threshold_handler(ctx: Context<Auth>, threshold: u64) -> Result<()> {
    require!(threshold > 0, ErrorCode::InvalidThreshold);
    if threshold > ctx.accounts.multisig.owners.len() as u64 {
        return Err(ErrorCode::InvalidThreshold.into());
    }
    let multisig = &mut ctx.accounts.multisig;
    multisig.threshold = threshold;
    Ok(())
}
