use {
    crate::{sort::sort_and_parse_token_accounts, TokenAccount},
    solana_pubkey::Pubkey,
    std::{
        collections::BTreeSet,
        fmt::{Debug, Display},
    },
    tracing::Level,
    url::Url,
    wasm_client_solana::{
        rpc_filter::TokenAccountsFilter,
        solana_account_decoder::parse_token::spl_token_ids,
        SolanaRpcClient,
    },
};

#[derive(Clone)]
pub struct TokenClient {
    pub(super) rpc: SolanaRpcClient,
    pub(super) owner: Pubkey,
    host: String,
}

impl Display for TokenClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_common())
    }
}
impl Debug for TokenClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_common())
    }
}

impl TokenClient {
    pub fn owner(&self) -> &Pubkey {
        &self.owner
    }

    pub fn rpc(&self) -> &SolanaRpcClient {
        &self.rpc
    }

    pub fn new(owner: &Pubkey, rpc: &SolanaRpcClient) -> Self {
        let host = Url::parse(&rpc.url()).ok();

        let host: String = host.map_or_else(
            || String::from("unknown-host"),
            |u| String::from(u.host_str().unwrap_or_default()),
        );
        Self {
            rpc: rpc.clone(),
            owner: *owner,
            host,
        }
    }

    fn fmt_common(&self) -> String {
        // format!("{} [{}]", self.owner, self.host)
        format!("[{}]", self.owner)
    }

    #[tracing::instrument(level = "info")]
    pub async fn tokens(&self) -> crate::Result<BTreeSet<TokenAccount>> {
        let mut token_accounts: BTreeSet<TokenAccount> = BTreeSet::new();

        for id in spl_token_ids() {
            let s = tracing::span!(
                Level::INFO,
                "token-accounts",
                program = id.to_string() // owner = self.to_string()
            );
            let ctx = s.enter();
            let results = self
                .rpc
                .get_token_accounts_by_owner(&self.owner, TokenAccountsFilter::ProgramId(id))
                .await?;

            tracing::debug!("tokens found {}", results.len());
            let mut accounts = sort_and_parse_token_accounts(&self.owner, results, true).accounts;
            token_accounts.append(&mut accounts);
            drop(ctx);
        }
        Ok(token_accounts)
    }
}
