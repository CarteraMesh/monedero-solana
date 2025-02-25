use {
    crate::TokenClient,
    solana_pubkey::Pubkey,
    solana_sdk::instruction::Instruction,
    spl_associated_token_account::{
        get_associated_token_address_with_program_id,
        instruction::create_associated_token_account,
    },
};

impl TokenClient {
    pub async fn create_account(
        &self,
        mint: &Pubkey,
        to: &Pubkey,
        program_id: &Pubkey,
    ) -> crate::Result<(Pubkey, Option<Instruction>)> {
        let to_address = get_associated_token_address_with_program_id(to, mint, program_id);
        let inst: Option<Instruction> = match self.rpc.get_account(&to_address).await.ok() {
            None => Some(create_associated_token_account(
                &self.owner,
                to,
                mint,
                program_id,
            )),
            Some(_) => None,
        };
        Ok((to_address, inst))
    }
}
