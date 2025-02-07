#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid solana public key {0:#?}")]
    InvalidAccount(String),

    #[error(transparent)]
    InvalidPubkey(#[from] solana_pubkey::ParsePubkeyError),

    #[error(transparent)]
    SerdeError(#[from] serde_json::error::Error),

    #[error(transparent)]
    SolanaProgramError(#[from] solana_program::program_error::ProgramError),

    #[error("spl-token program is not valid for this operation try spl-token-2022")]
    InvalidTokenProgram,

    #[error(transparent)]
    RpcRequestError(#[from] wasm_client_solana::ClientError),

    #[error("Account exists! {0}")]
    AccountExists(String),
    #[error("Invalid param for rpc {0}")]
    BadParameter(String),

    #[error("Token has non-zero value: {0}")]
    NonZero(String),

    #[error("{0}")]
    InvalidAmount(String),

    #[error("{0}")]
    InvalidMint(String),
}
