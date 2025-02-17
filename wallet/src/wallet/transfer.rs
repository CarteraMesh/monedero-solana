use solana_sdk::address_lookup_table::instruction;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token_2022::check_id as is_token_22;
use {
    super::SolanaWallet, crate::token_account::TokenAccount, solana_pubkey::Pubkey,
    solana_sdk::instruction::Instruction, solana_signature::Signature,
};

fn token_create_account_instruction(
    owner: &Pubkey,
    dest: &Pubkey,
    token: &TokenAccount,
) -> Instruction {
    let id = &token.program_id;
    create_associated_token_account(owner, dest, &token.account.mint, id)
}

fn token_tx_instruction(
    owner: &Pubkey,
    dest: &Pubkey,
    token: &TokenAccount,
    amt: u64,
) -> crate::Result<Instruction> {
    let id = &token.program_id;
    match is_token_22(id) {
        true => Ok(spl_token_2022::instruction::transfer(
            id,
            &token.address,
            dest,
            owner,
            &[owner],
            amt,
        )?),
        false => Ok(spl_token::instruction::transfer(
            id,
            &token.address,
            dest,
            owner,
            &[owner],
            amt,
        )?),
    }
}

impl SolanaWallet {
    pub async fn token_transfer(
        &self,
        token: &TokenAccount,
        to: &Pubkey,
        amt: u64,
    ) -> crate::Result<Signature> {
        let dest = get_associated_token_address(to, &token.account.mint);
        let mut instructions: Vec<Instruction> = Vec::new();
        if self.rpc.get_account(&dest).await.is_err() {
            instructions.push(token_create_account_instruction(&self.payer, &dest, token));
        }
        instructions.push(token_tx_instruction(&self.payer, &dest, token, amt)?);
        self.send_instructions(&instructions, None).await
    }

    pub async fn transfer(&self, to: &Pubkey, lamports: u64) -> crate::Result<Signature> {
        let ix = self.transfer_instructions(to, lamports);
        self.send_instructions(&ix, None).await
    }

    fn transfer_instructions(&self, to: &Pubkey, lamports: u64) -> Vec<Instruction> {
        vec![
            // spl_memo::build_memo(&self.memo, &[&self.sol_session.pk]),
            solana_sdk::system_instruction::transfer(&self.payer, to, lamports),
        ]
        //    //.with_memo(Some(&self.memo))
    }
}
