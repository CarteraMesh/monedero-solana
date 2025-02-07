mod setup;

use {
    monedero_solana_tokens::TokenAccount,
    setup::{config, TestConfig},
};

#[rstest::rstest]
fn close_non_zero_account(config: TestConfig) {
    let tokens: Vec<TokenAccount> =
        vec![setup::dummy_token(true, 123), setup::dummy_token(false, 0)];
    let inst = config.client.close_accounts(true, &tokens);
    assert!(inst.is_err());
}

#[rstest::rstest]
fn close_accounts(config: TestConfig) -> anyhow::Result<()> {
    let tokens: Vec<TokenAccount> = vec![setup::dummy_token(true, 0), setup::dummy_token(false, 0)];
    let inst = config.client.close_accounts(true, &tokens)?;
    assert_eq!(2, inst.len());
    assert!(inst.iter().any(|t| t.program_id == spl_token_2022::id()));
    assert!(inst.iter().any(|t| t.program_id == spl_token::id()));
    Ok(())
}
