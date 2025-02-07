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
        match &account.stake_state.delegated_vote_account_address {
            Some(_) => Ok(self.deactivate(&account.stake_pubkey)),
            None => Err(crate::Error::InvalidateState(format!(
                "{} is not delegated",
                account.stake_pubkey
            ))),
        }
    }

    pub fn withdraw_checked(&self, account: &KeyedStakeState) -> Result<Instruction> {
        match &account.stake_state.delegated_vote_account_address {
            None => Ok(self.withdraw(&account.stake_pubkey, account.stake_state.account_balance)),
            Some(_) => Err(crate::Error::InvalidateState(format!(
                "{} is delegated. please deactivate 1st",
                account.stake_pubkey
            ))),
        }
    }
}
