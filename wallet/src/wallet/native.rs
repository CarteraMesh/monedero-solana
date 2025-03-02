use {
    super::SolanaWallet,
    monedero_signer_sender::TransactionSignerSender,
    solana_signature::Signature,
};

impl<S: TransactionSignerSender> SolanaWallet<S> {
    pub async fn wrap(&self, amount: u64) -> crate::Result<Signature> {
        let instructions = self.tc.wrap(amount).await?;
        self.send_instructions(&instructions, None).await
    }

    pub async fn unwrap_sol(&self) -> crate::Result<Signature> {
        let instruction = self.tc.unwrap_sol()?;
        self.send_instructions(&[instruction], None).await
    }
}
