use {
    monedero_signer_sender::KeypairSender,
    solana_pubkey::Pubkey,
    std::sync::Once,
    tracing_subscriber::{fmt::format::FmtSpan, EnvFilter},
    wasm_client_solana::SolanaRpcClient,
};

pub const OWNER: Pubkey = Pubkey::from_str_const("215r9xfTFVYcE9g3fAUGowauM84egyUvFCbSo3LKNaep");
pub const USDC: Pubkey = Pubkey::from_str_const("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
pub const KEYPAIR: [u8; 64] = [
    186, 128, 232, 61, 254, 246, 197, 13, 125, 103, 212, 83, 16, 121, 144, 20, 93, 161, 35, 128,
    89, 135, 157, 200, 81, 159, 83, 204, 204, 130, 28, 42, 14, 225, 43, 2, 44, 16, 255, 214, 161,
    18, 184, 164, 253, 126, 16, 187, 134, 176, 75, 35, 179, 194, 181, 150, 67, 236, 131, 49, 45,
    155, 45, 253,
];

#[allow(clippy::missing_panics_doc)]
pub fn keypair_sender(rpc: Option<SolanaRpcClient>) -> KeypairSender {
    let rpc = rpc.unwrap_or_else(|| {
        let (_, r) = rpc_provider();
        r
    });
    KeypairSender::new(KEYPAIR.to_vec(), &rpc).expect("should not happen!")
}

pub fn rpc_provider() -> (String, SolanaRpcClient) {
    let url = std::env::var("RPC_URL").unwrap_or_else(|_| String::from(wasm_client_solana::DEVNET));
    let rpc = wasm_client_solana::SolanaRpcClient::new(&url);
    (url, rpc)
}

static INIT: Once = Once::new();

#[allow(clippy::unwrap_used, clippy::missing_panics_doc)]
pub fn setup() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_target(true)
            .with_level(true)
            .with_span_events(FmtSpan::CLOSE)
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        let env = dotenvy::dotenv();
        if env.is_err() {
            tracing::debug!("no .env file");
        }
    });
}
