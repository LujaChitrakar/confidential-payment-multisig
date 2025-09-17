use anchor_lang::prelude::*;
use std::convert::Into;
pub mod error;
pub mod instructions;
pub mod state;
use error::ErrorCode;
use instructions::{
    multisig::{
        approve::*, create_multisig::*, create_transaction::*, execute_transaction::*,
        set_owners_change_threshold::*,
    },
    payment_gateway::{
        add_kyc_entity::*, approve_large_transfer::*, create_transfer_request::*,
        emergency_freeze_entity::*, execute_transfer::*, initialize_gateway::*, register_bank::*,
    },
};
use state::*;
declare_id!("3j1ncRqK33zZfcD41825zgEErb6xQJJhrfSH2v5L11wj");

#[program]
pub mod multisig {

    use super::*;

    // MULTISIG
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

    //PAYMENT GATEWAY

    pub fn initialize_gateway(
        ctx: Context<InitializeGateway>,
        admin_multisig: Pubkey,
        compliance_multisig: Pubkey,
    ) -> Result<()> {
        initialize_gateway_handler(ctx, admin_multisig, compliance_multisig)?;
        Ok(())
    }

    pub fn add_kyc_entity(
        ctx: Context<AddKycEntity>,
        entity_id: String,
        entity_type: EntityType,
        compliance_score: u8,
        country_code: String,
    ) -> Result<()> {
        add_kyc_entity_handler(ctx, entity_id, entity_type, compliance_score, country_code)?;
        Ok(())
    }

    pub fn approve_large_transfer(ctx: Context<ApproveLargeTransfer>) -> Result<()> {
        approve_large_transfer_handler(ctx)?;
        Ok(())
    }

    pub fn create_transfer_request(
        ctx: Context<CreateTransferRequest>,
        transfer_id: String,
        recipient_bank: Pubkey,
        amount: u64,
        currency: String,
        reference: String,
        compliance_metadata: String,
    ) -> Result<()> {
        create_transfer_request_handler(
            ctx,
            transfer_id,
            recipient_bank,
            amount,
            currency,
            reference,
            compliance_metadata,
        )?;
        Ok(())
    }

    pub fn emergency_freeze_entity(
        ctx: Context<EmergencyFreezeEntity>,
        reason: String,
    ) -> Result<()> {
        emergency_freeze_entity_handler(ctx, reason)?;
        Ok(())
    }

    pub fn execute_transfer(ctx: Context<ExecuteTransfer>) -> Result<()> {
        execute_transfer_handler(ctx)?;
        Ok(())
    }

    pub fn register_bank(
        ctx: Context<RegisterBank>,
        bank_id: String,
        bank_name: String,
        swift_code: String,
        compliance_tier: u8,
    ) -> Result<()> {
        register_bank_handler(ctx, bank_id, bank_name, swift_code, compliance_tier)?;
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
fn validate_compliance(sender_kyc: &KycRecord, recipient_kyc: &KycRecord) -> Result<()> {
    require!(sender_kyc.is_active, ErrorCode::SenderNotVerified);
    require!(recipient_kyc.is_active, ErrorCode::RecipientNotVerified);
    require!(
        sender_kyc.compliance_score >= 70,
        ErrorCode::InsufficientComplianceScore
    );
    require!(
        recipient_kyc.compliance_score >= 70,
        ErrorCode::InsufficientComplianceScore
    );
    Ok(())
}

fn validate_sanctions(sender_kyc: &KycRecord, recipient_kyc: &KycRecord) -> Result<()> {
    let sanctioned_countries = vec!["IR", "KP", "SY"];
    require!(
        !sanctioned_countries.contains(&sender_kyc.country_code.as_str()),
        ErrorCode::SanctionedEntity
    );
    require!(
        !sanctioned_countries.contains(&recipient_kyc.country_code.as_str()),
        ErrorCode::SanctionedEntity
    );
    Ok(())
}
