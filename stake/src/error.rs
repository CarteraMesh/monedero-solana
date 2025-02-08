use solana_pubkey::Pubkey;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SolanaProgramError(#[from] solana_program::program_error::ProgramError),

    #[error("spl-token program is not valid for this operation try spl-token-2022")]
    InvalidTokenProgram,

    #[error(transparent)]
    RpcRequestError(#[from] wasm_client_solana::ClientError),

    #[error("Account exists! {0}")]
    AccountExists(String),

    #[error("Account not found {0}")]
    AccountNotFound(Pubkey),

    #[error("Invalid param for rpc {0}")]
    BadParameter(String),

    #[error("amount {amt} is not enough for minimum delegation {min_amt} ")]
    MinimumDelegation { amt: u64, min_amt: u64 },

    #[error(transparent)]
    PubkeyError(#[from] solana_pubkey::PubkeyError),

    #[error("{0}")]
    RpcError(String),

    #[error(transparent)]
    InstructionError(#[from] solana_program::instruction::InstructionError),

    #[error(transparent)]
    BincodeEncodeError(#[from] bincode::Error),
    #[error("{0}")]
    InvalidateState(String),
}
