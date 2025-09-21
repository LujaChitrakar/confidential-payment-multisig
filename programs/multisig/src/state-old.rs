// use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

// //MULTISIG
// #[account]
// // #[derive(InitSpace)]
// pub struct Multisig {
//     pub owners: Vec<Pubkey>,
//     pub threshold: u64,
//     pub nonce: u8,
//     pub owner_set_seqno: u32,
// }

// #[account]
// pub struct Transaction {
//     // The multisig account this transaction belongs to.
//     pub multisig: Pubkey,
//     // Target program to execute against.
//     pub program_id: Pubkey,
//     // Accounts requried for the transaction.
//     pub accounts: Vec<TransactionAccount>,
//     // Instruction data for the transaction.
//     pub data: Vec<u8>,
//     // signers[index] is true iff multisig.owners[index] signed the transaction.
//     pub signers: Vec<bool>,
//     // Boolean ensuring one time execution.
//     pub did_execute: bool,
//     // Owner set sequence number.
//     pub owner_set_seqno: u32,
// }

// impl From<&Transaction> for Instruction {
//     fn from(tx: &Transaction) -> Instruction {
//         Instruction {
//             program_id: tx.program_id,
//             accounts: tx.accounts.iter().map(Into::into).collect(),
//             data: tx.data.clone(),
//         }
//     }
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
// pub struct TransactionAccount {
//     pub pubkey: Pubkey,
//     pub is_signer: bool,
//     pub is_writable: bool,
// }

// impl From<&TransactionAccount> for AccountMeta {
//     fn from(account: &TransactionAccount) -> AccountMeta {
//         match account.is_writable {
//             false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
//             true => AccountMeta::new(account.pubkey, account.is_signer),
//         }
//     }
// }

// impl From<&AccountMeta> for TransactionAccount {
//     fn from(account_meta: &AccountMeta) -> TransactionAccount {
//         TransactionAccount {
//             pubkey: account_meta.pubkey,
//             is_signer: account_meta.is_signer,
//             is_writable: account_meta.is_writable,
//         }
//     }
// }

// //PAYMENT GATEWAY
// #[account]
// #[derive(InitSpace)]
// pub struct PaymentGateway {
//     pub admin: Pubkey,
//     pub treasury: Pubkey, //where fees go
//     pub fee_bps: u16,
//     pub is_active: bool,
//     pub total_banks: u32,
//     // pub token_mint:Pubkey,
// }

// #[account]
// #[derive(InitSpace)]
// pub struct BankAccount {
//     pub bank_id: u64,
//     #[max_len(20)]
//     pub bank_name:String,
//     pub bank_admin: Pubkey,
//     pub bank_multisig: Pubkey,
//     pub instant_withdrawl_limit: u64,
//     #[max_len(10)]
//     pub swift_code: String,
//     pub is_active: bool,
//     pub treasury_ata: Pubkey,
//     pub balance: u64,
// }

// #[account]
// #[derive(InitSpace)]
// pub struct KycRecord {
//     pub bank_id: u64,
//     pub is_active: bool,
//     pub verified_by: Pubkey,
//     pub updated_at: i64,
// }


// #[account]
// #[derive(InitSpace)]
// pub struct UserStats {
//     pub owner: Pubkey,
//     pub mint: Pubkey,
//     pub balance: u64,
//     pub bump: u8,
// }

// #[account]
// #[derive(InitSpace)]
// pub struct TransferRequest {
//     pub payer: Pubkey,
//     pub recipient_bank: Pubkey,
//     pub amount: u64,
//     pub token_mint: Pubkey,
//     pub status: TransferStatus,
//     pub created_at: i64,
//     // pub executed_at: Option<i64>,
//     pub required_multisig_approval: bool,
//     pub multisig_approved: bool,
//     pub nonce: u64,
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
// pub enum TransferStatus {
//     Pending,
//     Approved,
//     Executed,
//     Rejected,
//     Frozen,
// }
