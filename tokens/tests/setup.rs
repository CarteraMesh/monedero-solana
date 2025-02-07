use {
    monedero_solana_tokens::{TokenAccount, TokenClient, TokenMetadata},
    solana_sdk::{signature::Keypair, signer::Signer},
    wasm_client_solana::solana_account_decoder::parse_token::{
        UiAccountState,
        UiTokenAccount,
        UiTokenAmount,
    },
};

#[allow(dead_code)]
pub fn dummy_token(legacy: bool, amt: u64) -> TokenAccount {
    let kp = Keypair::new();
    let zero = UiTokenAmount {
        ui_amount: None,
        decimals: 0,
        amount: format!("{amt}"),
        ui_amount_string: format!("{amt}"),
    };
    let program_id = if legacy {
        spl_token::id()
    } else {
        spl_token_2022::id()
    };
    TokenAccount {
        address: kp.pubkey(),
        program_id,
        is_associated: true,
        account: UiTokenAccount {
            mint: kp.pubkey(),
            owner: kp.pubkey(),
            token_amount: zero,
            delegate: None,
            state: UiAccountState::Frozen,
            is_native: false,
            rent_exempt_reserve: None,
            delegated_amount: None,
            close_authority: None,
            extensions: [].to_vec(),
        },
        has_permanent_delegate: false,
        metadata: TokenMetadata::default(),
    }
}

#[rstest::fixture]
pub fn config() -> TestConfig {
    test_utils::setup();
    TestConfig::new()
}

pub struct TestConfig {
    pub client: TokenClient,
    #[allow(dead_code)]
    pub rpc: wasm_client_solana::SolanaRpcClient,
}

impl TestConfig {
    fn new() -> Self {
        let (_, rpc) = test_utils::rpc_provider();
        let tc = TokenClient::new(&test_utils::OWNER, &rpc);
        Self { client: tc, rpc }
    }
}
