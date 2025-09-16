use anchor_lang::prelude::*;
use std::convert::Into;
pub mod error;
pub mod instructions;
pub mod state;
use error::ErrorCode;
use instructions::{
    approve::*, create_multisig::*, create_transaction::*, execute_transaction::*,
    set_owners_change_threshold::*,
};
use state::*;
declare_id!("3j1ncRqK33zZfcD41825zgEErb6xQJJhrfSH2v5L11wj");

#[program]
pub mod multisig {
    use super::*;

    pub fn create_multisig(
        ctx: Context<CreateMultisig>,
        owners: Vec<Pubkey>,
        threshold: u64,
        nonce: u8,
    ) -> Result<()> {
        create_multisig_handler(ctx, owners, threshold, nonce)?;
        Ok(())
    }

    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        pid: Pubkey,
        accs: Vec<TransactionAccount>,
        data: Vec<u8>,
    ) -> Result<()> {
        create_transaction_handler(ctx, pid, accs, data)?;
        Ok(())
    }

    pub fn approve(ctx: Context<Approve>) -> Result<()> {
        approve_handler(ctx)?;
        Ok(())
    }

    pub fn set_owners_change_threshold<'info>(
        ctx: Context<'_, '_, '_, 'info, Auth<'info>>,
        owners: Vec<Pubkey>,
        threshold: u64,
    ) -> Result<()> {
        set_owners_and_change_threshold_handler(ctx, owners, threshold)?;
        Ok(())
    }

    pub fn set_owners(ctx: Context<Auth>, owners: Vec<Pubkey>) -> Result<()> {
        set_owners_handler(ctx, owners)?;
        Ok(())
    }

    pub fn change_threshold(ctx: Context<Auth>, threshold: u64) -> Result<()> {
        change_threshold_handler(ctx, threshold)?;
        Ok(())
    }

    pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
        execute_transaction_handler(ctx)?;
        Ok(())
    }
}

pub fn assert_unique_owners(owners: &[Pubkey]) -> Result<()> {
    for (i, owner) in owners.iter().enumerate() {
        require!(
            !owners.iter().skip(i + 1).any(|item| item == owner),
            ErrorCode::UniqueOwners
        )
    }
    Ok(())
}
