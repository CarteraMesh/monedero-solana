#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid solana public key {0:#?}")]
    InvalidAccount(String),

    #[error(transparent)]
    RpcRequestError(#[from] wasm_client_solana::ClientError),

    #[error("sysvar clock account not found {0}")]
    ClockAccountNotFound(solana_pubkey::Pubkey),

    #[error("clock account not valid")]
    InvalidClockAccount,
}
