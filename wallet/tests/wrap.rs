mod setup;
use {
    monedero_solana::{sol_to_lamports, TokenAccount},
    setup::{config, TestConfig},
    std::collections::BTreeSet,
};

fn wrapped_amount(tokens: &BTreeSet<TokenAccount>) -> u64 {
    let w_sol = tokens
        .iter()
        .find(|t| t.account.mint == spl_token::native_mint::id());
    w_sol.map_or(0, monedero_solana::TokenAccount::amount)
}
#[tokio::test]
#[rstest::rstest]
async fn token_wrap_unwrap(#[future] config: TestConfig) -> anyhow::Result<()> {
    let config = config.await;
    let wallet = &config.wallet;
    let prev_amt = wrapped_amount(&wallet.tokens().await?);
    let lamports = sol_to_lamports(0.00325);
    let sig = wallet.wrap(lamports).await?;
    TestConfig::explorer(sig);
    wallet.rpc().wait_for_new_block(50).await?;
    let new_amt = wrapped_amount(&wallet.tokens().await?);
    assert_eq!(prev_amt + lamports, new_amt);

    TestConfig::explorer(wallet.unwrap_sol().await?);
    wallet.rpc().wait_for_new_block(50).await?;
    let new_amt = wrapped_amount(&wallet.tokens().await?);
    assert_eq!(prev_amt, new_amt);
    Ok(())
}
