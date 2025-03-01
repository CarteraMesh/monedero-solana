mod error;
mod lookup;
mod memo;
pub use error::Error;
use {
    monedero_jup_ag::JupiterInstructor,
    monedero_solana_stake::StakeClient,
    monedero_solana_tokens::TokenClient,
};
pub type Result<T> = std::result::Result<T, Error>;
pub use {
    monedero_jup_ag as jup_ag,
    monedero_solana_stake as stake,
    monedero_solana_tokens as token,
};
use {solana_pubkey::Pubkey, wasm_client_solana::SolanaRpcClient};

#[derive(Clone)]
pub struct Instructor {
    token_client: TokenClient,
    stake_client: StakeClient,
    jup_client: JupiterInstructor,
}

impl Instructor {
    pub fn jup_client(&self) -> &JupiterInstructor {
        &self.jup_client
    }

    pub fn token_client(&self) -> &TokenClient {
        &self.token_client
    }

    pub fn stake_client(&self) -> &StakeClient {
        &self.stake_client
    }

    pub fn rpc(&self) -> &SolanaRpcClient {
        self.token_client.rpc()
    }

    pub fn payer(&self) -> &Pubkey {
        self.token_client.owner()
    }

    pub fn new(payer: &Pubkey, rpc: &SolanaRpcClient) -> Self {
        let token_client = TokenClient::new(payer, rpc);
        let stake_client = StakeClient::new(payer, rpc);
        Self {
            token_client,
            stake_client,
            jup_client: JupiterInstructor::new(payer),
        }
    }
}
