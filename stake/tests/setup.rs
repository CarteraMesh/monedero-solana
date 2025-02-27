use monedero_solana_stake::StakeClient;

#[rstest::fixture]
pub fn config() -> TestConfig {
    test_utils::setup();
    TestConfig::new()
}

pub struct TestConfig {
    pub client: StakeClient,
    #[allow(dead_code)]
    pub rpc: wasm_client_solana::SolanaRpcClient,
}

impl TestConfig {
    fn new() -> Self {
        let (_, rpc) = test_utils::rpc_provider();
        let tc = StakeClient::new(&test_utils::OWNER, &rpc);
        tracing::info!("created stake client {tc}");
        Self { client: tc, rpc }
    }
}
