use {
    crate::{KeyedStakeState, Result, StakeClient},
    solana_program::{instruction::Instruction, pubkey::Pubkey},
    solana_sdk::stake::instruction::{self as stake_instruction},
};

impl StakeClient {
    pub fn deactivate(&self, account: &Pubkey) -> Instruction {
        stake_instruction::deactivate_stake(account, &self.owner)
    }

    pub fn withdraw(&self, account: &Pubkey, lamports: u64) -> Instruction {
        stake_instruction::withdraw(account, &self.owner, &self.owner, lamports, None)
    }

    pub fn deactivate_checked(&self, account: &KeyedStakeState) -> Result<Instruction> {
        match account.stake_state.condition {
            crate::StakeCondition::Delegated => Ok(self.deactivate(&account.stake_pubkey)),
            _ => Err(crate::Error::InvalidateState(format!(
                "{} is {}. please deactivate 1st",
                account.stake_pubkey, account.stake_state.condition
            ))),
        }
    }

    pub fn withdraw_checked(&self, account: &KeyedStakeState) -> Result<Instruction> {
        match account.stake_state.condition {
            crate::StakeCondition::Idle => {
                Ok(self.withdraw(&account.stake_pubkey, account.stake_state.account_balance))
            }
            _ => Err(crate::Error::InvalidateState(format!(
                "{} is {}. please deactivate 1st",
                account.stake_pubkey, account.stake_state.condition
            ))),
        }
    }
}
