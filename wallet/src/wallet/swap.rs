use {
    super::SolanaWallet,
    monedero_signer_sender::TransactionSignerSender,
    monedero_solana_instructor::jup_ag,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
};

impl<S: TransactionSignerSender> SolanaWallet<S> {
    pub async fn swap_sol_jup(&self, to: &Pubkey, amount: u64) -> crate::Result<Signature> {
        self.swap_jup(&spl_token::native_mint::id(), to, amount, false)
            .await
    }

    pub async fn swap_jup(
        &self,
        from: &Pubkey,
        to: &Pubkey,
        amount: u64,
        wrap_unwrap_sol: bool,
    ) -> crate::Result<Signature> {
        let jup = self.instructor.jup_client();
        let quote = jup_ag::QuoteConfig {
            slippage_bps: Some(50),
            ..Default::default()
        };
        let (instructions, lookups) = jup.swap(from, to, amount, quote, wrap_unwrap_sol).await?;
        self.send_instructions(&instructions, Some(lookups)).await
    }
}
