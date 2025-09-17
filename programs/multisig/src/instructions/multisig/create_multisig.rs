use crate::{assert_unique_owners, error::ErrorCode, state::Multisig};
use anchor_lang::prelude::*;

// Initializes a new multisig account with a set of owners and a threshold.
#[derive(Accounts)]
pub struct CreateMultisig<'info> {
    #[account(zero, signer)]
    multisig: Box<Account<'info, Multisig>>,
}

pub fn create_multisig_handler(
    ctx: Context<CreateMultisig>,
    owners: Vec<Pubkey>,
    threshold: u64,
    nonce: u8,
) -> Result<()> {
    assert_unique_owners(&owners)?;
    require!(
        threshold > 0 && threshold <= owners.len() as u64,
        ErrorCode::InvalidThreshold
    );
    require!(!owners.is_empty(), ErrorCode::InvalidOwnersLen);

    let multisig = &mut ctx.accounts.multisig;
    multisig.owners = owners;
    multisig.threshold = threshold;
    multisig.nonce = nonce;
    multisig.owner_set_seqno = 0;
    Ok(())
}
