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
    #[msg("Invalid Bank Id")]
    InvalidBankId,

    //payment errors
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

    //from here
    #[msg("Owner Already Exists")]
    OwnerAlreadyExists,
    #[msg("InactiveStratum")]
    InactiveStratum,
    #[msg("InvalidOwnersCount")]
    InvalidOwnersCount,
    #[msg("ThresholdExceeds")]
    ThresholdExceeds,
    #[msg("OwnerDoesntExist")]
    OwnerDoesntExist,
    #[msg("InvalidStrataLen")]
    InvalidStrataLen,
    #[msg("CannotDeactivateFirst")]
    CannotDeactivateFirst,
    #[msg("ThresholdNotZero")]
    ThresholdNotZero,
    #[msg("ActiveStratum")]
    ActiveStratum,
    #[msg("ThresholdZero")]
    ThresholdZero,
    #[msg("InvalidTransaction")]
    InvalidTransaction,
    #[msg("NotVoteStatus")]
    NotVoteStatus,
    #[msg("VersionOutdated")]
    VersionOutdated,
    #[msg("InvalidStratumNumber")]
    InvalidStratumNumber,
    #[msg("InsufficientVotes")]
    InsufficientVotes,
    #[msg("AlreadyFinalized")]
    AlreadyFinalized,
    #[msg("InvalidTxData")]
    InvalidTxData,
    #[msg("InvalidCreator")]
    InvalidCreator,
    #[msg("InvalidTxDataIndex")]
    InvalidTxDataIndex,
    #[msg("InvalidNameLen")]
    InvalidNameLen,
    #[msg("DuplicateOwner")]
    DuplicateOwner,
    #[msg("InvalidDescriptionLen")]
    InvalidDescriptionLen,
    #[msg("NotAccepted")]
    NotAccepted,
    #[msg("InvalidInstructionAccount")]
    InvalidInstructionAccount,
    #[msg("InvalidInstruction")]
    InvalidInstruction,
    #[msg("InsufficientQuoteTokens")]
    InsufficientQuoteTokens,
    #[msg("InvalidSwap")]
    InvalidSwap,
    #[msg("KycNotVerified")]
    KycNotVerified,
    #[msg("InvalidAdmin")]
    InvalidAdmin,
}
