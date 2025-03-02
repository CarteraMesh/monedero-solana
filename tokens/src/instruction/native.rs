use {
    crate::TokenClient,
    solana_sdk::instruction::Instruction,
    spl_associated_token_account::get_associated_token_address_with_program_id,
};

impl TokenClient {
    #[tracing::instrument(level = "info")]
    pub async fn wrap(&self, amount: u64) -> crate::Result<Vec<Instruction>> {
        let token_id = spl_token::id();
        let (account, create) = self
            .create_account(&spl_token::native_mint::id(), &self.owner, &token_id)
            .await?;
        let transfer = solana_sdk::system_instruction::transfer(&self.owner, &account, amount);
        let sync = spl_token::instruction::sync_native(&token_id, &account)?;
        let mut instructions: Vec<Instruction> = Vec::with_capacity(3);
        if let Some(i) = create {
            instructions.push(i);
        }
        instructions.push(transfer);
        instructions.push(sync);
        Ok(instructions)
    }

    #[tracing::instrument(level = "info")]
    pub fn unwrap_sol(&self) -> crate::Result<Instruction> {
        let token_id = spl_token::id();
        let owner = &self.owner;
        let account = get_associated_token_address_with_program_id(
            owner,
            &spl_token::native_mint::id(),
            &token_id,
        );
        Ok(spl_token::instruction::close_account(
            &token_id,
            &account,
            owner,
            owner,
            &[owner],
        )?)
    }
}
