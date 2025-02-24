use {
    crate::{TokenAccount, TokenClient},
    solana_pubkey::Pubkey,
    solana_sdk::instruction::Instruction,
};

impl TokenClient {
    #[tracing::instrument(level = "info")]
    pub async fn transfer(
        &self,
        token: &TokenAccount,
        to: &Pubkey,
        amt: u64,
    ) -> crate::Result<Vec<Instruction>> {
        let mint = &token.account.mint;
        let mut instructions = Vec::with_capacity(2);
        let (to_address, create_ins) = self.create_account(mint, to, &token.program_id).await?;

        if let Some(i) = create_ins {
            instructions.push(i);
        }

        let i = spl_token_2022::instruction::transfer_checked(
            &token.program_id,
            &token.address,
            mint,
            &to_address,
            &self.owner,
            &[&self.owner],
            amt,
            token.account.token_amount.decimals,
        )?;
        instructions.push(i);
        Ok(instructions)
    }
}
