mod setup;

use setup::{config, TestConfig};

#[tokio::test]
#[rstest::rstest]
async fn tokens(config: TestConfig) -> anyhow::Result<()> {
    let tokens = config.client.tokens().await?;
    assert!(!tokens.is_empty());
    Ok(())
}
