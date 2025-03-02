#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid solana public key {0:#?}")]
    InvalidAccount(String),

    #[error("current wallet-connect does not have solana namespace")]
    NoSolanaNamespace,

    #[error("current wallet-connect does not have solana accounts")]
    SolanaAccountNotFound,

    #[error(transparent)]
    InvalidPubkey(#[from] solana_pubkey::ParsePubkeyError),

    #[error(transparent)]
    BincodeEncodeError(#[from] bincode::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::error::Error),

    #[error("error decoding bs58: #{0}")]
    Bs58Error(String),

    #[error("failed to load keypair from bytes")]
    KeyPairFailure,

    #[error(transparent)]
    Base64Error(#[from] base64::DecodeError),

    #[error(transparent)]
    SignerError(#[from] solana_sdk::signature::SignerError),
    // SignerError(#[from] solana_signer::SignerError),
    #[error("invalid signature. Length is not 64 '{0}'")]
    SigError(String),

    #[error(transparent)]
    PubkeyError(#[from] solana_pubkey::PubkeyError),

    #[error(transparent)]
    SolanaProgramError(#[from] solana_program::program_error::ProgramError),
    // #[error(transparent)]
    // TransactionError(#[from] solana_sdk::transaction::TransactionError),
    //
    // #[error(transparent)]
    // InstructionError(#[from] solana_program::instruction::InstructionError),
    //
    // #[error(transparent)]
    // TokenError(#[from] spl_token_client::token::TokenError),
    #[error("signature failed to confirm {0}")]
    ConfirmationFailure(solana_signature::Signature),

    #[error("spl-token program is not valid for this operation try spl-token-2022")]
    InvalidTokenProgram,

    #[error(transparent)]
    RpcRequestError(#[from] wasm_client_solana::ClientError),

    #[error("Account exists! {0}")]
    AccountExists(String),
    #[error("Invalid param for rpc {0}")]
    BadParameter(String),

    //#[error(transparent)]
    // StorageError(#[from] monedero_store::Error),

    //#[error(transparent)]
    // XdgError(#[from] microxdg::XdgError),
    #[error("amount {amt} is not enough for minimum delegation {min_amt} ")]
    MinimumDelegation { amt: u64, min_amt: u64 },

    #[error("lookup table {0} is not initialized")]
    UninitializedLookupTable(String),

    #[error(transparent)]
    CompileError(#[from] solana_sdk::message::CompileError),

    #[error("TokenAmount is zero {0}")]
    InvalidTokenAmount(String),

    #[error(transparent)]
    InstructionError(#[from] monedero_solana_instructor::Error),

    #[error(transparent)]
    TokenInstructionError(#[from] monedero_solana_instructor::token::Error),

    #[error("Simulation failure {0}")]
    SimulateError(String),

    #[error("Lookup table address not foud {0}")]
    LookupTableNotFound(solana_pubkey::Pubkey),

    #[error("Decode error for lookup table {0}")]
    LookupTableDecodeError(String),

    #[error(transparent)]
    StakeError(#[from] monedero_solana_instructor::stake::Error),

    #[error(transparent)]
    JupiterError(#[from] monedero_solana_instructor::jup_ag::Error),
}
