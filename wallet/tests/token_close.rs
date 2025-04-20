mod setup;
use {
    monedero_solana::TokenAccount,
    setup::{config, TestConfig},
    std::collections::BTreeSet,
};

fn find_mint(tokens: &BTreeSet<TokenAccount>) -> (TokenAccount, TokenAccount) {
    let legacy = tokens.iter().find(|t| t.account.mint == setup::MINT_LEGACY);
    assert!(legacy.is_some());
    let legacy = legacy.unwrap();

    let ext = tokens.iter().find(|t| t.account.mint == setup::MINT_EXT);
    assert!(ext.is_some());
    let ext = ext.unwrap();

    (legacy.clone(), ext.clone())
}

#[tokio::test]
#[rstest::rstest]
async fn token_close_burn(#[future] config: TestConfig) -> anyhow::Result<()> {
    let config = config.await;
    let wallet = &config.wallet;
    let sig = wallet
        .token_mint(&setup::MINT_LEGACY, &spl_token::id(), 123)
        .await?;
    TestConfig::explorer(sig);
    let sig = wallet
        .token_mint(&setup::MINT_EXT, &spl_token_2022::id(), 123)
        .await?;
    TestConfig::explorer(sig);

    wallet.rpc().wait_for_new_block(254).await?;
    let tokens = wallet.tokens().await?;
    let (legacy, ext) = find_mint(&tokens);
    let sig = wallet.token_burn(&legacy, 11).await?;
    TestConfig::explorer(sig);
    let sig = wallet.token_burn(&ext, 11).await?;
    TestConfig::explorer(sig);

    // find new token amounts so we can burn them all
    wallet.rpc().wait_for_new_block(254).await?;
    let tokens = wallet.tokens().await?;
    let (legacy, ext) = find_mint(&tokens);

    #[allow(clippy::tuple_array_conversions)]
    let sig = wallet.token_burn_close(&[legacy, ext]).await?;
    TestConfig::explorer(sig);
    // wallet.rpc().wait_for_new_block(70).await?;
    // let sig = wallet.token_close(&[legacy]).await?;
    // TestConfig::explorer(sig);
    Ok(())
}
