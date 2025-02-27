use {
    super::SolanaWallet,
    monedero_signer_sender::TransactionSignerSender,
    monedero_solana_instructor::stake::{KeyedStakeState, StakeClient},
    solana_pubkey::Pubkey,
    solana_sdk::{signature::Keypair, signer::Signer},
    solana_signature::Signature,
};

impl<S: TransactionSignerSender> SolanaWallet<S> {
    pub fn stake_client(&self) -> &StakeClient {
        self.instructor.stake_client()
    }

    pub async fn stake_accounts(&self) -> crate::Result<Vec<KeyedStakeState>> {
        let sc = self.instructor.stake_client();
        Ok(sc.accounts().await?)
    }

    #[tracing::instrument(level = "info")]
    pub async fn stake_withdraw(&self, account: &KeyedStakeState) -> crate::Result<Signature> {
        let sc = self.instructor.stake_client();
        let instruction = vec![sc.withdraw_checked(account)?];
        self.send_instructions(&instruction, None).await
    }

    #[tracing::instrument(level = "info")]
    pub async fn stake_deactivate(&self, account: &KeyedStakeState) -> crate::Result<Signature> {
        let sc = self.instructor.stake_client();
        let instruction = vec![sc.deactivate_checked(account)?];
        self.send_instructions(&instruction, None).await
    }

    #[tracing::instrument(level = "info")]
    pub async fn stake_create(&self, lamports: u64) -> crate::Result<(Pubkey, Signature)> {
        let sc = self.instructor.stake_client();
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("failed to get system time")
            .as_secs()
            .to_string();
        let (account, instructions) = sc.create_account(seed, lamports).await?;
        let sig = self.send_instructions(&instructions, None).await?;
        Ok((account, sig))
    }

    #[tracing::instrument(level = "info")]
    pub async fn stake_create_and_delegate(
        &self,
        validator: &Pubkey,
        lamports: u64,
    ) -> crate::Result<(Pubkey, Signature)> {
        let sc = self.instructor.stake_client();
        let kp = Keypair::new();
        let stake_account = kp.pubkey();
        let instructions = sc
            .create_delegate(&stake_account, validator, lamports)
            .await?;
        let sig = self
            .send_instructions_with_signer(&instructions, &[kp])
            .await?;
        Ok((stake_account, sig))
    }
}
