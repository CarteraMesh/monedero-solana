use {
    crate::{TokenAccount, TokenClient},
    solana_pubkey::Pubkey,
    solana_sdk::instruction::Instruction,
    spl_associated_token_account::{
        get_associated_token_address_with_program_id,
        instruction::create_associated_token_account,
    },
};

impl TokenClient {
    pub async fn transfer(
        &self,
        token: &TokenAccount,
        to: &Pubkey,
        amt: u64,
    ) -> crate::Result<Vec<Instruction>> {
        let mint = &token.account.mint;
        let mut instructions = Vec::new();
        let to_address = get_associated_token_address_with_program_id(to, mint, &token.program_id);
        let exists = self.rpc.get_account(&to_address).await.ok();
        if exists.is_none() {
            instructions.push(create_associated_token_account(
                &self.owner,
                to,
                mint,
                &token.program_id,
            ));
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
