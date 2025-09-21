use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct PaymentGateway {
    pub admin: Pubkey,
    pub treasury: Pubkey, //where fees go
    pub fee_bps: u16,
    pub is_active: bool,
    pub total_banks: u32,
    // pub token_mint:Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct BankAccount {
    pub bank_id: u64,
    #[max_len(20)]
    pub bank_name: String,
    pub bank_admin: Pubkey,
    pub bank_multisig: Pubkey,
    pub instant_withdrawl_limit: u64,
    #[max_len(10)]
    pub swift_code: String,
    pub is_active: bool,
    pub treasury_ata: Pubkey,
    pub balance: u64,
}

#[account]
#[derive(InitSpace)]
pub struct KycRecord {
    pub bank_id: u64,
    pub is_active: bool,
    pub verified_by: Pubkey,
    pub updated_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct UserStats {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub balance: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct TransferRequest {
    pub payer: Pubkey,
    pub recipient_bank: Pubkey,
    pub amount: u64,
    pub token_mint: Pubkey,
    pub status: TransferStatus,
    pub created_at: i64,
    // pub executed_at: Option<i64>,
    pub required_multisig_approval: bool,
    pub multisig_approved: bool,
    pub nonce: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum TransferStatus {
    Pending,
    Approved,
    Executed,
    Rejected,
    Frozen,
}
