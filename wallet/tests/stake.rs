mod setup;
use {
    setup::{config, TestConfig},
    solana_pubkey::Pubkey,
    solana_sdk::native_token::sol_to_lamports,
};

const VALIDATOR: Pubkey = Pubkey::from_str_const("7AETLyAGJWjp6AWzZqZcP362yv5LQ3nLEdwnXNjdNwwF");

#[tokio::test]
#[rstest::rstest]
async fn stake_deactivate(#[future] config: TestConfig) -> anyhow::Result<()> {
    let w = &config.await.wallet;
    let account = w
        .stake_accounts()
        .await?
        .into_iter()
        .find(|s| s.stake_state.deactivating_stake == 0 && s.stake_state.active_stake > 0);
    assert!(account.is_some());
    let account = account.unwrap();
    TestConfig::explorer(w.stake_deactivate(&account).await?);
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn stake_withdraw(#[future] config: TestConfig) -> anyhow::Result<()> {
    let w = &config.await.wallet;
    let accounts = w.stake_accounts_undelegated().await?;
    if accounts.is_empty() {
        tracing::warn!("no stake accounts to withdraw");
        return Ok(());
    }
    TestConfig::explorer(w.stake_withdraw(&accounts[0]).await?);
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn stake_create(#[future] config: TestConfig) -> anyhow::Result<()> {
    let w = &config.await.wallet;
    let (account, sig) = w.stake_create(sol_to_lamports(2.1)).await?;
    tracing::info!("new stake account {account}");
    TestConfig::explorer(sig);
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn stake_delegate_create(#[future] config: TestConfig) -> anyhow::Result<()> {
    let w = &config.await.wallet;
    let (account, sig) = w
        .stake_create_and_delegate(&VALIDATOR, sol_to_lamports(2.1))
        .await?;
    tracing::info!("new stake account {account}");
    TestConfig::explorer(sig);
    Ok(())
}
