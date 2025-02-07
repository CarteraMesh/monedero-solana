mod setup;

use setup::{config, TestConfig};

#[tokio::test]
#[rstest::rstest]
async fn accounts(config: TestConfig) -> anyhow::Result<()> {
    let c = &config.client;
    let accouts = c.accounts().await?;
    let undelegated = c.accounts_undelegated().await?;
    assert!(accouts.len() >= undelegated.len());
    Ok(())
}
