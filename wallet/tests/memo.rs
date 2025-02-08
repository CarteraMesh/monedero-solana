mod setup;

use setup::{config, TestConfig};

#[tokio::test]
#[rstest::rstest]
async fn memo(#[future] config: TestConfig) -> anyhow::Result<()> {
    let w = &config.await.wallet;
    TestConfig::explorer(w.memo("blahblah").await?);
    Ok(())
}
