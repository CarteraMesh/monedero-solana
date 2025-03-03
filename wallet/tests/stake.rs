mod setup;
use {
    setup::{config, TestConfig},
    solana_pubkey::Pubkey,
    solana_sdk::native_token::sol_to_lamports,
};

const VALIDATOR: Pubkey = Pubkey::from_str_const("vgcDar2pryHvMgPkKaZfh8pQy4BJxv7SpwUG7zinWjG");

#[tokio::test]
#[rstest::rstest]
async fn stake_deactivate(#[future] config: TestConfig) -> anyhow::Result<()> {
    let w = &config.await.wallet;
    let sc = w.stake_client();
    let accounts = sc.accounts_delegated().await?;
    let account = accounts.first();
    if account.is_none() {
        tracing::warn!("no stakes accouts to deactivate");
        return Ok(());
    }
    let account = account.unwrap();
    TestConfig::explorer(w.stake_deactivate(account).await?);
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn stake_withdraw(#[future] config: TestConfig) -> anyhow::Result<()> {
    let w = &config.await.wallet;
    let sc = w.stake_client();
    let accounts = sc.accounts_idle().await?;
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
    let (account, sig) = w.stake_create(sol_to_lamports(0.1)).await?;
    tracing::info!("new stake account {account}");
    TestConfig::explorer(sig);
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn stake_delegate_create(#[future] config: TestConfig) -> anyhow::Result<()> {
    let w = &config.await.wallet;
    let (account, sig) = w
        .stake_create_and_delegate(&VALIDATOR, sol_to_lamports(0.1))
        .await?;
    tracing::info!("new stake account {account}");
    TestConfig::explorer(sig);
    Ok(())
}
