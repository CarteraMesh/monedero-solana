mod setup;

use setup::{config, TestConfig};

#[rstest::rstest]
fn burn(config: TestConfig) -> anyhow::Result<()> {
    let token = setup::dummy_token(true, 100);
    let i = config.client.burn_account(&token, 1)?;
    assert_eq!(i.program_id, token.program_id);
    Ok(())
}

#[rstest::rstest]
fn burn_more_than_allowed(config: TestConfig) {
    let token = setup::dummy_token(false, 100);
    let i = config.client.burn_account(&token, 101);
    assert!(i.is_err());
}
