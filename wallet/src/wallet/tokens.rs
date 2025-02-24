use {
    super::SolanaWallet,
    monedero_signer_sender::TransactionSignerSender,
    monedero_solana_instructor::token::TokenAccount,
    solana_pubkey::Pubkey,
    solana_sdk::instruction::Instruction,
    solana_signature::Signature,
    std::collections::BTreeSet,
};

impl<S: TransactionSignerSender + Send> SolanaWallet<S> {
    pub async fn token_mint(
        &self,
        mint: &Pubkey,
        program_id: &Pubkey,
        amount: u64,
    ) -> crate::Result<Signature> {
        let ins = self.tc.mint(mint, self.pk(), program_id, amount).await?;
        self.send_instructions(&ins, None).await
    }

    pub async fn tokens(&self) -> crate::Result<BTreeSet<TokenAccount>> {
        Ok(self.tc.tokens().await?)
    }

    pub async fn token_burn(&self, token: &TokenAccount, amt: u64) -> crate::Result<Signature> {
        let i = self.tc.burn_account(token, amt)?;
        self.send_instructions(&[i], None).await
    }

    pub async fn token_close(&self, tokens: &[TokenAccount]) -> crate::Result<Signature> {
        let i = self.tc.close_accounts(false, tokens)?;
        self.send_instructions(&i, None).await
    }

    pub async fn token_burn_close(&self, tokens: &[TokenAccount]) -> crate::Result<Signature> {
        let mut instructions: Vec<Instruction> = Vec::with_capacity(tokens.len() * 2);
        for t in tokens {
            if t.amount() > 0 {
                instructions.push(self.tc.burn_account(t, t.amount())?);
            }
        }
        instructions.append(&mut self.tc.close_accounts(false, tokens)?);
        self.send_instructions(&instructions, None).await
    }

    pub async fn token_transfer(
        &self,
        token: &TokenAccount,
        to: &Pubkey,
        amt: u64,
    ) -> crate::Result<Signature> {
        let instructions = self.tc.transfer(token, to, amt).await?;
        self.send_instructions(&instructions, None).await
    }
}
