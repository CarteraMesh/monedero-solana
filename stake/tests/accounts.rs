mod setup;

use setup::{config, TestConfig};

#[tokio::test]
#[rstest::rstest]
async fn accounts(config: TestConfig) -> anyhow::Result<()> {
    let c = &config.client;
    let accounts = c.accounts().await?;
    let undelegated = c.accounts_idle().await?;
    assert!(accounts.len() >= undelegated.len());
    Ok(())
}
