use {
    crate::{TokenAccount, TokenClient},
    solana_sdk::instruction::Instruction,
};

impl TokenClient {
    pub fn close_accounts(
        &self,
        check_amount: bool,
        tokens: &[TokenAccount],
    ) -> crate::Result<Vec<Instruction>> {
        let mut to_close = Vec::with_capacity(tokens.len());
        for t in tokens {
            to_close.push(self.close_account(check_amount, t)?);
        }
        Ok(to_close)
    }

    #[tracing::instrument(level = "info")]
    pub fn close_account(
        &self,
        check_amount: bool,
        token: &TokenAccount,
    ) -> crate::Result<Instruction> {
        if check_amount && token.amount() > 0 {
            return Err(crate::Error::NonZero(token.to_string()));
        }
        if spl_token::native_mint::check_id(&token.account.mint) {
            return Err(crate::Error::InvalidMint(
                "native mint (wrapped sol) cannot be used".to_string(),
            ));
        }
        let owner = &self.owner;
        let id = &token.program_id;
        Ok(spl_token_2022::instruction::close_account(
            id,
            &token.address,
            owner,
            owner,
            &[owner],
        )?)
    }
}
