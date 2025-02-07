use {
    crate::{
        Error::{AccountExists, BadParameter, MinimumDelegation},
        Result, StakeClient,
    },
    solana_program::{
        instruction::Instruction,
        pubkey::Pubkey,
        stake::state::{Authorized, StakeStateV2},
    },
    solana_sdk::stake::{
        self,
        instruction::{self as stake_instruction},
    },
};

impl StakeClient {
    pub async fn create_account(
        &self,
        seed: impl AsRef<str> + Send,
        lamports: u64,
    ) -> Result<(Pubkey, Vec<Instruction>)> {
        let min_amt = self.minimum_delegation().await?;
        if lamports < min_amt {
            return Err(MinimumDelegation {
                amt: lamports,
                min_amt,
            });
        }
        let stake_account = self.owner();
        let stake_account_address =
            Pubkey::create_with_seed(stake_account, seed.as_ref(), &stake::program::id())?;
        if self.rpc.get_account(&stake_account_address).await.is_ok() {
            return Err(AccountExists(stake_account_address.to_string()));
        }

        let minimum_balance = self
            .rpc
            .get_minimum_balance_for_rent_exemption(StakeStateV2::size_of())
            .await?;
        if lamports < minimum_balance {
            return Err(BadParameter(format!(
                "need at least {minimum_balance} lamports for stake account to be rent exempt, \
                 provided lamports: {lamports}"
            )));
        }

        let authorized = Authorized {
            staker: *stake_account,
            withdrawer: *stake_account,
        };
        let inxs = stake_instruction::create_account_with_seed_checked(
            stake_account,
            &stake_account_address,
            stake_account,
            seed.as_ref(),
            &authorized,
            lamports,
        );
        Ok((stake_account_address, inxs))
    }
}
