use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The given owner is not part of this multisig.")]
    InvalidOwner,
    #[msg("Owners length must be non zero.")]
    InvalidOwnersLen,
    #[msg("Not enough owners signed this transaction.")]
    NotEnoughSigners,
    #[msg("Cannot delete a transaction that has been signed by an owner.")]
    TransactionAlreadySigned,
    #[msg("Overflow when adding.")]
    Overflow,
    #[msg("Cannot delete a transaction the owner did not create.")]
    UnableToDelete,
    #[msg("The given transaction has already been executed.")]
    AlreadyExecuted,
    #[msg("Threshold must be less than or equal to the number of owners.")]
    InvalidThreshold,
    #[msg("Owners must be unique")]
    UniqueOwners,

    //payment errors
    #[msg("Invalid bank ID")]
    InvalidBankId,
    #[msg("Invalid bank name")]
    InvalidBankName,
    #[msg("Invalid SWIFT code")]
    InvalidSwiftCode,
    #[msg("Invalid compliance tier")]
    InvalidComplianceTier,
    #[msg("Invalid entity ID")]
    InvalidEntityId,
    #[msg("Invalid compliance score")]
    InvalidComplianceScore,
    #[msg("Invalid country code")]
    InvalidCountryCode,
    #[msg("Invalid transfer ID")]
    InvalidTransferId,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid currency")]
    InvalidCurrency,
    #[msg("Transfer not approved")]
    TransferNotApproved,
    #[msg("Multisig approval required for this transfer")]
    MultisigApprovalRequired,
    #[msg("Approval not required for this transfer")]
    ApprovalNotRequired,
    #[msg("Invalid transfer status")]
    InvalidTransferStatus,
    #[msg("Sender not verified")]
    SenderNotVerified,
    #[msg("Recipient not verified")]
    RecipientNotVerified,
    #[msg("Insufficient compliance score")]
    InsufficientComplianceScore,
    #[msg("Sanctioned entity")]
    SanctionedEntity,
}
