use {
    monedero_signer_sender::KeypairSender,
    monedero_solana::{SolanaWallet, TokenAccount},
    solana_pubkey::Pubkey,
    solana_signature::Signature,
};

#[allow(dead_code)]
pub const LOOKUP: Pubkey = Pubkey::from_str_const("4VdGS3365Jqa2WGRUVpnSkpTVvzgHECdVYtbysSsEzj1");
#[allow(dead_code)]
pub const TO: Pubkey = Pubkey::from_str_const("E4SfgGV2v9GLYsEkCQhrrnFbBcYmAiUZZbJ7swKGzZHJ");

#[rstest::fixture]
pub async fn config() -> TestConfig {
    test_utils::setup();
    TestConfig::new().await
}

pub struct TestConfig {
    pub wallet: SolanaWallet<KeypairSender>,
    #[allow(dead_code)]
    pub rpc: wasm_client_solana::SolanaRpcClient,
}

impl TestConfig {
    async fn new() -> Self {
        let (url, rpc) = test_utils::rpc_provider();
        tracing::info!("using url {url}");
        let wallet =
            SolanaWallet::with_lookup(test_utils::keypair_sender(Some(rpc.clone())), &rpc, &LOOKUP)
                .await
                .expect("failed to init wallet");

        Self { wallet, rpc }
    }

    #[allow(dead_code)]
    pub async fn tokens_non_empty(&self) -> anyhow::Result<Vec<TokenAccount>> {
        let tokens: Vec<TokenAccount> = self
            .wallet
            .tokens()
            .await?
            .into_iter()
            .filter(|t| t.amount() > 0)
            .collect();
        Ok(tokens)
    }

    pub fn explorer(sig: Signature) {
        // ctx.clip
        //    .set_contents(format!("{sig}"))
        //    .expect("Failed to set clipboard");
        //
        tracing::info!("\n{sig}\nhttps://solscan.io/tx/{sig}?cluster=devnet\nhttps://solana.fm/tx/{sig}?cluster=devnet\nhttps://xray.helius.dev/tx/{sig}?cluster=devnet");
    }
}
