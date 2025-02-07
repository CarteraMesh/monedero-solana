use {
    super::{Result, StakeClient},
    crate::{KeyedStakeState, StakeState},
    solana_program::{
        clock::{Clock, Epoch, Slot},
        feature::Feature,
        pubkey::Pubkey,
        stake::state::{Meta, StakeActivationStatus, StakeStateV2},
        stake_history::StakeHistory,
        sysvar::{self, stake_history},
    },
    solana_sdk::{
        account::{from_account, ReadableAccount},
        account_utils::StateMut,
    },
    wasm_client_solana::{
        rpc_filter::{Memcmp, RpcFilterType},
        rpc_response::RpcVoteAccountInfo,
        solana_account_decoder, RpcAccountInfoConfig, RpcProgramAccountsConfig, SolanaRpcClient,
    },
};

async fn get_feature_activation_slot(
    rpc: &SolanaRpcClient,
    feature_id: &Pubkey,
) -> crate::Result<Option<Slot>> {
    let feature_account = rpc.get_account(feature_id).await?;
    let decoded: Feature = bincode::deserialize(feature_account.data())?;
    Ok(decoded.activated_at)
}

const FEATURE_SET_ID: Pubkey =
    Pubkey::from_str_const("GwtDQBghCTBgmX2cpEGNPxTEBUTQRaDMGTr5qychdGMj");

impl StakeClient {
    #[tracing::instrument(level = "info")]
    pub async fn validators(&self) -> Result<Vec<RpcVoteAccountInfo>> {
        let delegators = self.rpc.get_vote_accounts().await?;
        Ok(delegators.current)
    }

    pub async fn accounts_undelegated(&self) -> Result<Vec<KeyedStakeState>> {
        Ok(self
            .accounts()
            .await?
            .into_iter()
            .filter(|a| a.stake_state.delegated_vote_account_address.is_none())
            .collect())
    }

    #[tracing::instrument(level = "info")]
    pub async fn accounts(&self) -> crate::Result<Vec<KeyedStakeState>> {
        let id = solana_sdk::stake::program::id();
        let program_accounts_config = RpcProgramAccountsConfig {
            account_config: RpcAccountInfoConfig {
                encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
                ..RpcAccountInfoConfig::default()
            },
            filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new_base58_encoded(
                44,
                self.owner.as_ref(),
            ))]),
            ..RpcProgramAccountsConfig::default()
        };
        let all_stake_accounts = self
            .rpc
            .get_program_accounts_with_config(&id, program_accounts_config)
            .await?;
        let stake_history_account = self.rpc.get_account(&stake_history::id()).await?;
        let clock_account = self.rpc.get_account(&sysvar::clock::id()).await?;
        let clock: Clock = from_account(&clock_account).ok_or_else(|| {
            crate::Error::RpcError("Failed to deserialize clock sysvar".to_string())
        })?;
        let stake_history: StakeHistory =
            from_account(&stake_history_account).ok_or_else(|| {
                crate::Error::RpcError("Failed to deserialize state history".to_string())
            })?;

        let new_rate_activation_epoch =
            get_feature_activation_slot(&self.rpc, &FEATURE_SET_ID).await?;

        let mut stake_accounts: Vec<KeyedStakeState> = vec![];
        for (stake_pubkey, stake_account) in all_stake_accounts {
            let stake_state = stake_account.state()?;
            match stake_state {
                StakeStateV2::Initialized(_) | StakeStateV2::Stake(_, _, _) => {
                    stake_accounts.push(KeyedStakeState {
                        stake_pubkey,
                        stake_state: build_stake_state(
                            stake_account.lamports,
                            &stake_state,
                            &stake_history,
                            &clock,
                            new_rate_activation_epoch,
                        ),
                    });
                }
                _ => {}
            }
        }
        Ok(stake_accounts)
    }
}

#[allow(clippy::if_not_else)]
fn build_stake_state(
    account_balance: u64,
    stake_state: &StakeStateV2,
    stake_history: &StakeHistory,
    clock: &Clock,
    new_rate_activation_epoch: Option<Epoch>,
) -> StakeState {
    match stake_state {
        StakeStateV2::Stake(
            Meta {
                rent_exempt_reserve,
                authorized: _,
                lockup,
            },
            stake,
            _,
        ) => {
            let current_epoch = clock.epoch;
            let StakeActivationStatus {
                effective,
                activating,
                deactivating,
            } = stake.delegation.stake_activating_and_deactivating(
                current_epoch,
                stake_history,
                new_rate_activation_epoch,
            );
            let lockup = if lockup.is_in_force(clock, None) {
                Some(lockup.into())
            } else {
                None
            };
            StakeState {
                stake_type: super::StakeType::Stake,
                account_balance,
                credits_observed: stake.credits_observed,
                delegated_stake: stake.delegation.stake,
                delegated_vote_account_address: if stake.delegation.voter_pubkey
                    != Pubkey::default()
                {
                    Some(stake.delegation.voter_pubkey.to_string())
                } else {
                    None
                },
                activation_epoch: if stake.delegation.activation_epoch < u64::MAX {
                    stake.delegation.activation_epoch
                } else {
                    0
                },
                deactivation_epoch: if stake.delegation.deactivation_epoch < u64::MAX {
                    stake.delegation.deactivation_epoch
                } else {
                    0
                },
                lockup,
                current_epoch,
                rent_exempt_reserve: *rent_exempt_reserve,
                active_stake: effective,
                activating_stake: activating,
                deactivating_stake: deactivating,
                ..StakeState::default()
            }
        }
        StakeStateV2::RewardsPool => StakeState {
            stake_type: super::StakeType::RewardsPool,
            account_balance,
            ..StakeState::default()
        },
        StakeStateV2::Uninitialized => StakeState {
            account_balance,
            ..StakeState::default()
        },
        StakeStateV2::Initialized(Meta {
            rent_exempt_reserve,
            authorized,
            lockup,
        }) => {
            let lockup = if lockup.is_in_force(clock, None) {
                Some(lockup.into())
            } else {
                None
            };
            StakeState {
                stake_type: super::StakeType::Initialized,
                account_balance,
                credits_observed: 0,
                authorized: Some(*authorized),
                lockup,
                rent_exempt_reserve: *rent_exempt_reserve,
                ..StakeState::default()
            }
        }
    }
}
