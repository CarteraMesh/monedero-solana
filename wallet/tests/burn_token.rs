mod setup;
use {
    setup::{config, TestConfig},
    solana_pubkey::Pubkey,
};

// USDC
const USDC: Pubkey = Pubkey::from_str_const("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
#[tokio::test]
#[rstest::rstest]
async fn token_burn(#[future] config: TestConfig) -> anyhow::Result<()> {
    let w = &config.await.wallet;
    let token = w
        .tokens()
        .await?
        .into_iter()
        .find(|t| t.amount() > 0 && !t.account.is_native)
        .expect("no tokens found with amount > 0");
    TestConfig::explorer(w.token_burn(&token, 1).await?);
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn token_burn_and_close(#[future] config: TestConfig) -> anyhow::Result<()> {
    let w = &config.await.wallet;
    let token = w
        .tokens()
        .await?
        .into_iter()
        .find(|t| t.account.mint == USDC);

    match token {
        None => tracing::warn!("no USDC to burn/close"),
        Some(t) => {
            tracing::info!("burning / closing USDC  {t}");
            TestConfig::explorer(w.token_burn_close(&[t]).await?);
        }
    };
    Ok(())
}
