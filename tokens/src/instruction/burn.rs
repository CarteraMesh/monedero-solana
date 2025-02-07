use {
    crate::{TokenAccount, TokenClient},
    solana_sdk::instruction::Instruction,
};

impl TokenClient {
    #[tracing::instrument(level = "info")]
    pub fn burn_account(&self, token: &TokenAccount, amt: u64) -> crate::Result<Instruction> {
        if amt > token.amount() {
            return Err(crate::Error::InvalidAmount(format!(
                "amount to burn ({amt}) is greater than token amount {token}"
            )));
        }
        Ok(spl_token_2022::instruction::burn(
            &token.program_id,
            &token.address,
            &token.account.mint,
            &self.owner,
            &[],
            amt,
        )?)
    }
}
