use {
    serde::{Deserialize, Serialize},
    solana_program::{
        clock::{Epoch, Slot, UnixTimestamp},
        pubkey::Pubkey,
        stake::state::{Authorized, Lockup},
    },
    solana_sdk::native_token::lamports_to_sol,
    std::fmt::{Debug, Display, Formatter},
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum StakeType {
    Stake,
    RewardsPool,
    Uninitialized,
    Initialized,
}

impl Display for StakeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stake => write!(f, "Stake"),
            Self::RewardsPool => write!(f, "RewardsPool"),
            Self::Uninitialized => write!(f, "Uninitialized"),
            Self::Initialized => write!(f, "Initialized"),
        }
    }
}
impl Default for StakeType {
    fn default() -> Self {
        Self::Uninitialized
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct StakeLockup {
    pub unix_timestamp: UnixTimestamp,
    pub epoch: Epoch,
    pub custodian: String,
}

impl From<&Lockup> for StakeLockup {
    fn from(lockup: &Lockup) -> Self {
        Self {
            unix_timestamp: lockup.unix_timestamp,
            epoch: lockup.epoch,
            custodian: lockup.custodian.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EpochReward {
    pub epoch: Epoch,
    pub effective_slot: Slot,
    pub amount: u64,       // lamports
    pub post_balance: u64, // lamports
    pub percent_change: f64,
    pub apr: Option<f64>,
    pub commission: Option<u8>,
    pub block_time: UnixTimestamp,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StakeState {
    pub stake_type: StakeType,
    pub account_balance: u64,
    pub credits_observed: u64,
    pub delegated_stake: u64,
    pub delegated_vote_account_address: Option<String>,
    pub activation_epoch: Epoch,
    pub deactivation_epoch: Epoch,
    pub lockup: Option<StakeLockup>,
    pub authorized: Option<Authorized>,
    pub current_epoch: Epoch,
    pub rent_exempt_reserve: u64,
    pub active_stake: u64,
    pub activating_stake: u64,
    pub deactivating_stake: u64,
    pub epoch_rewards: Option<Vec<EpochReward>>,
    pub condition: StakeCondition,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum StakeCondition {
    Idle,
    Deactivating,
    #[default]
    Delegated,
}

impl Display for StakeCondition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let condition: String = match self {
            Self::Idle => "idle".to_owned(),
            Self::Deactivating => "dectivating".to_owned(),
            Self::Delegated => "delegated".to_owned(),
        };
        write!(f, "{condition}")
    }
}

impl StakeState {
    pub fn stake_condition(stake: &Self) -> StakeCondition {
        if stake.delegated_vote_account_address.is_none() {
            return StakeCondition::Idle;
        }
        if stake.deactivation_epoch != 0 && stake.current_epoch > stake.deactivation_epoch {
            return StakeCondition::Idle;
        }

        if stake.deactivation_epoch == 0 {
            return StakeCondition::Delegated;
        }

        StakeCondition::Deactivating
    }
}

impl Display for StakeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = self
            .delegated_vote_account_address
            .as_ref()
            .map_or_else(String::new, |address| format!(" vote:{address}"));
        write!(
            f,
            "condition:{} balance:{} type:{} {}",
            self.condition,
            lamports_to_sol(self.account_balance),
            self.stake_type,
            v,
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyedStakeState {
    pub stake_pubkey: Pubkey,
    #[serde(flatten)]
    pub stake_state: StakeState,
}

impl Debug for KeyedStakeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} state={},amount={}",
            self.stake_pubkey, self.stake_state, self.stake_state.account_balance
        )
    }
}
impl Display for KeyedStakeState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.stake_pubkey, self.stake_state)
    }
}
