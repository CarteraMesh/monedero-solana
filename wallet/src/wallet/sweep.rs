use super::SolanaWallet;
use crate::token_account::TokenAccount;
use solana_pubkey::Pubkey;
use solana_sdk::instruction::Instruction;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token_2022::check_id as is_token_22;

fn close_instruction(owner: &Pubkey, token: &TokenAccount) -> crate::Result<Instruction> {
    let id = &token.program_id;
    match is_token_22(id) {
        true => Ok(spl_token_2022::instruction::close_account(
            id,
            &token.address,
            owner,
            owner,
            &[owner],
        )?),
        false => Ok(spl_token::instruction::close_account(
            id,
            &token.address,
            owner,
            owner,
            &[owner],
        )?),
    }
}

fn tx_instruction(
    owner: &Pubkey,
    dest: &Pubkey,
    token: &TokenAccount,
) -> crate::Result<Instruction> {
    let id = &token.program_id;
    let amt = token.amount();
    if amt == 0 {
        return Err(crate::Error::InvalidTokenAmount(token.clone()));
    }
    match is_token_22(id) {
        true => Ok(spl_token_2022::instruction::transfer_checked(
            id,
            &token.address,
            &token.account.mint,
            dest,
            owner,
            &[owner],
            amt,
            token.account.token_amount.decimals,
        )?),
        false => Ok(spl_token::instruction::transfer_checked(
            id,
            &token.address,
            &token.account.mint,
            dest,
            owner,
            &[owner],
            amt,
            token.account.token_amount.decimals,
        )?),
    }
}

fn create_account_instruction(owner: &Pubkey, dest: &Pubkey, token: &TokenAccount) -> Instruction {
    let id = &token.program_id;
    create_associated_token_account(owner, dest, &token.account.mint, id)
}

impl SolanaWallet {
    pub fn sweep_tokens<'a>(
        &'a self,
        dest: &Pubkey,
        tokens: impl Iterator<Item = &'a TokenAccount>,
        //tokens: &[TokenAccount],
    ) -> crate::Result<Vec<Instruction>> {
        let mut create_instructions: Vec<Instruction> = Vec::with_capacity(100);
        let mut tx_instructions: Vec<Instruction> = Vec::with_capacity(30);
        let mut close_instructions: Vec<Instruction> = Vec::with_capacity(30);
        let owner = &self.payer;

        for token_account in tokens {
            let dest_token_account =
                get_associated_token_address(dest, &token_account.account.mint);

            if token_account.amount() > 0 {
                create_instructions.push(create_account_instruction(owner, dest, token_account));
                tx_instructions.push(tx_instruction(owner, &dest_token_account, token_account)?);
            }
            close_instructions.push(close_instruction(owner, token_account)?);
        }
        tracing::info!(
            "creating/transfer {}/{} tokens. closing {}",
            create_instructions.len(),
            tx_instructions.len(),
            close_instructions.len()
        );
        create_instructions.append(&mut tx_instructions);
        //create_instructions.append(&mut close_instructions);
        Ok(create_instructions)
    }
}
