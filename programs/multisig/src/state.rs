use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

//MULTISIG
#[account]
// #[derive(InitSpace)]
pub struct Multisig {
    pub owners: Vec<Pubkey>,
    pub threshold: u64,
    pub nonce: u8,
    pub owner_set_seqno: u32,
}

#[account]
pub struct Transaction {
    // The multisig account this transaction belongs to.
    pub multisig: Pubkey,
    // Target program to execute against.
    pub program_id: Pubkey,
    // Accounts requried for the transaction.
    pub accounts: Vec<TransactionAccount>,
    // Instruction data for the transaction.
    pub data: Vec<u8>,
    // signers[index] is true iff multisig.owners[index] signed the transaction.
    pub signers: Vec<bool>,
    // Boolean ensuring one time execution.
    pub did_execute: bool,
    // Owner set sequence number.
    pub owner_set_seqno: u32,
}

impl From<&Transaction> for Instruction {
    fn from(tx: &Transaction) -> Instruction {
        Instruction {
            program_id: tx.program_id,
            accounts: tx.accounts.iter().map(Into::into).collect(),
            data: tx.data.clone(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl From<&TransactionAccount> for AccountMeta {
    fn from(account: &TransactionAccount) -> AccountMeta {
        match account.is_writable {
            false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
            true => AccountMeta::new(account.pubkey, account.is_signer),
        }
    }
}

impl From<&AccountMeta> for TransactionAccount {
    fn from(account_meta: &AccountMeta) -> TransactionAccount {
        TransactionAccount {
            pubkey: account_meta.pubkey,
            is_signer: account_meta.is_signer,
            is_writable: account_meta.is_writable,
        }
    }
}

//PAYMENT GATEWAY
#[account]
#[derive(InitSpace)]
pub struct PaymentGateway {
    pub admin_multisig: Pubkey,
    pub compliance_multisig: Pubkey,
    pub is_active: bool,
    pub total_banks: u32,
    pub large_transfer_threshold: u64,
    pub max_daily_volume: u64,
}

#[account]
#[derive(InitSpace)]
pub struct BankAccount {
    #[max_len(30)]
    pub bank_id: String,
    #[max_len(20)]
    pub bank_name: String,
    #[max_len(10)]
    pub swift_code: String,
    pub compliance_tier: u8,
    pub is_active: bool,
    pub kyc_authority: Pubkey,
    pub treasury_vault: Pubkey,
    pub total_transfers: u64,
    pub total_volume: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct KycRecord {
    #[max_len(10)]
    pub entity_id: String,
    // pub entity_type: EntityType,
    pub compliance_score: u8,
    #[max_len(10)]
    pub country_code: String,
    pub is_active: bool,
    pub verified_by: Pubkey,
    pub verification_timestamp: i64,
    #[max_len(20)]
    pub freeze_reason: Option<String>,
    pub frozen_at: Option<i64>,
}

#[account]
#[derive(InitSpace)]
pub struct TransferRequest {
    #[max_len(20)]
    pub transfer_id: String,
    pub sender_bank: Pubkey,
    pub recipient_bank: Pubkey,
    pub sender_entity: Pubkey,
    pub recipient_entity: Pubkey,
    pub amount: u64,
    #[max_len(20)]
    pub currency: String,
    #[max_len(20)]
    pub reference: String,
    #[max_len(20)]
    pub compliance_metadata: String,
    pub status: TransferStatus,
    pub created_at: i64,
    pub executed_at: Option<i64>,
    pub required_multisig_approval: bool,
    pub multisig_approved: bool,
}

// Enums and Events
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EntityType {
    Individual,
    Corporate,
    Institution,
    Government,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum TransferStatus {
    Pending,
    Approved,
    Executed,
    Rejected,
    Frozen,
}
