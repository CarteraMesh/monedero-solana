mod setup;

use {
    monedero_solana_stake::{KeyedStakeState, StakeCondition, StakeState},
    setup::{config, TestConfig},
    solana_pubkey::Pubkey,
};
#[rstest::rstest]
fn accounts(config: TestConfig) -> anyhow::Result<()> {
    let c = &config.client;
    let dummy = Pubkey::default();
    let state = StakeState {
        condition: StakeCondition::Idle,
        ..Default::default()
    };
    let mut account = KeyedStakeState {
        stake_pubkey: dummy,
        stake_state: state,
    };
    c.withdraw_checked(&account)?;

    account.stake_state.condition = StakeCondition::Delegated;

    assert!(c.withdraw_checked(&account).is_err());
    c.deactivate_checked(&account)?;

    account.stake_state.condition = StakeCondition::Deactivating;
    assert!(c.deactivate_checked(&account).is_err());

    tracing::info!("{}", account.stake_state);
    Ok(())
}
