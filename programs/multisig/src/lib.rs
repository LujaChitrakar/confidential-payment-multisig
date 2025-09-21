use anchor_lang::prelude::*;
use std::convert::Into;
pub mod error;
pub mod instructions;
pub mod state;
use instructions::{
    multisig::{
        accept_transaction::*, add_tx_data::*, change_multisig::*, change_multisig_realloc::*,
        create_multisig::*, create_transaction::*, create_tx_data::*, create_versioned_tx_data::*,
        execute_transaction::*, finalize_tx_data::*, vote_transaction::*,
    },
    payment_gateway::{
        admin::{attest_kyc::*, initialize_gateway::*, register_bank::*},
        bank::{bank_deposit::*, bank_withdraw::*, emergency_freeze::*},
    },
};
use state::{
    multisig::{ChangeReallocType, ChangeType, Stratum},
    tx_data::{InstructionAccount, InstructionData, VersionedInstructionData},
};

use crate::error::ErrorCode;

declare_id!("3j1ncRqK33zZfcD41825zgEErb6xQJJhrfSH2v5L11wj");

#[program]
pub mod multisig {
    use super::*;

    // MULTISIGpub
    pub fn create_multisig(
        ctx: Context<CreateMultisig>,
        strata: Vec<Stratum>,
        name: String,
    ) -> Result<()> {
        create_multisig_handler(ctx, strata, name)
    }

    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        owner_stratum: u8,
        description: String,
    ) -> Result<()> {
        create_transaction_handler(ctx, owner_stratum, description)
    }

    pub fn create_tx_data(
        ctx: Context<CreateTxData>,
        instructions: Vec<InstructionData>,
    ) -> Result<()> {
        create_tx_data_handler(ctx, instructions)
    }

    pub fn create_versioned_tx_data(
        ctx: Context<CreateVersionedTxData>,
        versioned_ixs: Vec<VersionedInstructionData>,
        lookup_table: Option<Pubkey>,
    ) -> Result<()> {
        create_versioned_tx_data_handler(ctx, versioned_ixs, lookup_table)
    }

    pub fn add_tx_data(
        ctx: Context<AddTxData>,
        keys: Vec<InstructionAccount>,
        instruction_index: u8,
    ) -> Result<()> {
        add_tx_data_handler(ctx, keys, instruction_index)
    }

    pub fn finalize_tx_data(ctx: Context<FinalizeTxData>) -> Result<()> {
        finalize_tx_data_handler(ctx)
    }

    pub fn vote_transaction(
        ctx: Context<VoteTransaction>,
        owner_stratum: u8,
        is_accept: bool,
    ) -> Result<()> {
        vote_transaction_handler(ctx, owner_stratum, is_accept)
    }

    pub fn accept_transaction(ctx: Context<AcceptTransaction>, owner_stratum: u8) -> Result<()> {
        accept_transaction_handler(ctx, owner_stratum)
    }

    pub fn execute_transaction<'info>(
        ctx: Context<'_, '_, '_, 'info, ExecuteTransaction<'info>>,
        owner_stratum: u8,
    ) -> Result<()> {
        execute_transaction_handler(ctx, owner_stratum)
    }

    pub fn change_multisig(
        ctx: Context<ChangeMultisig>,
        change_type: ChangeType,
        stratum: u8,
    ) -> Result<()> {
        change_multisig_handler(ctx, change_type, stratum)
    }

    pub fn change_multisig_realloc(
        ctx: Context<ChangeMultisigRealloc>,
        change_type: ChangeReallocType,
    ) -> Result<()> {
        change_multisig_realloc_handler(ctx, change_type)
    }

    //     //PAYMENT GATEWAY

    pub fn initialize_gateway(ctx: Context<InitializeGateway>, admin: Pubkey) -> Result<()> {
        initialize_gateway_handler(ctx, admin)?;
        Ok(())
    }

    pub fn attest_kyc(ctx: Context<AttestKyc>, bank_id: u64) -> Result<()> {
        attest_kyc_handler(ctx, bank_id)?;
        Ok(())
    }

    pub fn register_bank(
        ctx: Context<RegisterBank>,
        bank_id: u64,
        bank_name: String,
        swift_code: String,
    ) -> Result<()> {
        register_bank_handler(ctx, bank_id, bank_name, swift_code)?;
        Ok(())
    }

    pub fn bank_deposit(ctx: Context<BankDeposit>, bank_id: u64, amount: u64) -> Result<()> {
        deposit_handler(ctx, amount)?;
        Ok(())
    }

    pub fn bank_withdraw(ctx: Context<BankWithdraw>, bank_id: u64, amount: u64) -> Result<()> {
        withdraw_handler(ctx, amount)?;
        Ok(())
    }

    pub fn emergency_freeze(ctx: Context<EmergencyFreeze>, bank_id: u64) -> Result<()> {
        emergency_freeze_handler(ctx, bank_id)?;
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
