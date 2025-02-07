mod setup;

use setup::{config, TestConfig};
use solana_pubkey::Pubkey;

use monedero_solana_stake::{KeyedStakeState, StakeState};
#[rstest::rstest]
fn accounts(config: TestConfig) -> anyhow::Result<()> {
    let c = &config.client;
    let dummy = Pubkey::default();
    let state = StakeState::default();
    let mut account = KeyedStakeState {
        stake_pubkey: dummy,
        stake_state: state,
    };
    c.withdraw_checked(&account)?;

    account.stake_state.delegated_vote_account_address = Some(dummy.to_string());

    assert!(c.withdraw_checked(&account).is_err());
    c.deactivate_checked(&account)?;

    account.stake_state.delegated_vote_account_address = None;
    assert!(c.deactivate_checked(&account).is_err());
    Ok(())
}
