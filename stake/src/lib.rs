mod error;
pub use error::Error;
use solana_pubkey::Pubkey;
mod account;
mod client;
mod create;
mod delegate;
mod withdrawal;
pub use account::*;
use {
    std::fmt::{Debug, Display, Formatter},
    wasm_client_solana::SolanaRpcClient,
};
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct StakeClient {
    rpc: SolanaRpcClient,
    owner: Pubkey,
}

impl Display for StakeClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_common(f)
    }
}
impl Debug for StakeClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_common(f)
    }
}

impl StakeClient {
    pub fn new(owner: &Pubkey, rpc: &SolanaRpcClient) -> Self {
        Self {
            owner: *owner,
            rpc: rpc.clone(),
        }
    }

    pub async fn minimum_delegation(&self) -> Result<u64> {
        Ok(self.rpc.get_stake_minimum_delegation().await?)
    }

    pub fn owner(&self) -> &Pubkey {
        &self.owner
    }

    fn fmt_common(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[StakeClient][{}]", self.owner)
    }
}
