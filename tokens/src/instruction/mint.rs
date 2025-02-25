use {crate::TokenClient, solana_pubkey::Pubkey, solana_sdk::instruction::Instruction};

impl TokenClient {
    pub async fn mint(
        &self,
        mint: &Pubkey,
        to: &Pubkey,
        program_id: &Pubkey,
        amount: u64,
    ) -> crate::Result<Vec<Instruction>> {
        let (to_address, create_ins) = self.create_account(mint, to, program_id).await?;
        let mut instructions = Vec::new();
        if let Some(i) = create_ins {
            instructions.push(i);
        }
        let i = spl_token_2022::instruction::mint_to(
            program_id,
            mint,
            &to_address,
            &self.owner,
            &[&self.owner],
            amount,
        )?;
        instructions.push(i);
        Ok(instructions)
    }
}
