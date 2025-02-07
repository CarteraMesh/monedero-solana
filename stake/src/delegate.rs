use {
    crate::{Error::RpcError, Result, StakeClient},
    solana_program::{instruction::Instruction, pubkey::Pubkey},
    solana_sdk::stake::{
        instruction::{self as stake_instruction},
        state::{Authorized, Lockup},
    },
    wasm_client_solana::{rpc_response::RpcVoteAccountStatus, RpcGetVoteAccountsConfig},
};

const DELINQUENT_VALIDATOR_SLOT_DISTANCE: u64 = 128;
impl StakeClient {
    async fn validate(&self, vote_account: &Pubkey) -> Result<()> {
        let get_vote_accounts_config = RpcGetVoteAccountsConfig {
            vote_pubkey: Some(*vote_account),
            keep_unstaked_delinquents: Some(true),
            ..RpcGetVoteAccountsConfig::default()
        };
        let RpcVoteAccountStatus {
            current,
            delinquent,
        } = self
            .rpc
            .get_vote_accounts_with_config(get_vote_accounts_config)
            .await?;
        // filter should return at most one result
        let rpc_vote_account = current
            .first()
            .or_else(|| delinquent.first())
            .ok_or(crate::Error::AccountNotFound(*vote_account))?;

        let activated_stake = rpc_vote_account.activated_stake;
        let root_slot = rpc_vote_account.root_slot;
        let min_root_slot = self.rpc.get_slot().await?;
        let min_root_slot = min_root_slot.saturating_sub(DELINQUENT_VALIDATOR_SLOT_DISTANCE);
        let sanity_check_result = if root_slot >= min_root_slot || activated_stake == 0 {
            Ok(())
        } else if root_slot == 0 {
            return Err(RpcError(
                "Unable to delegate. Vote account has no root slot".to_string(),
            ));
        } else {
            Err(RpcError(format!(
                "Unable to delegate.  Vote account appears delinquent because its current root \
                 slot, {root_slot}, is less than {min_root_slot}"
            )))
        };
        sanity_check_result?;
        Ok(())
    }
    pub async fn delegate(
        &self,
        stake_account: &Pubkey,
        vote_account: &Pubkey,
    ) -> Result<Vec<Instruction>> {
        self.validate(vote_account).await?;
        Ok(vec![stake_instruction::delegate_stake(
            stake_account,
            &self.owner,
            vote_account,
        )])
    }
    pub async fn create_delegate(
        &self,
        stake_pubkey: &Pubkey,
        vote_account: &Pubkey,
        lamports: u64,
    ) -> Result<Vec<Instruction>> {
        let auth: Authorized = Authorized::auto(self.owner());
        self.validate(vote_account).await?;
        let mut instructions = solana_sdk::stake::instruction::create_account(
            self.owner(),
            stake_pubkey,
            &auth,
            &Lockup::default(),
            lamports,
        );
        instructions.push(solana_sdk::stake::instruction::delegate_stake(
            stake_pubkey,
            self.owner(),
            vote_account,
        ));
        Ok(instructions)
    }
}
