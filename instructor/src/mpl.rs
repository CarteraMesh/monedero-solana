use crate::Instructor;
use solana_pubkey::Pubkey;
use solana_sdk::instruction::Instruction;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_associated_token_account_client::address::get_associated_token_address;

impl Instructor {
    pub fn mpl_transfer(&self, to: &Pubkey, token: &Pubkey mint: &Pubkey) -> Vec<Instruction> {
        let mut builder = mpl_token_metadata::instructions::TransferBuilder::new();
        builder.token(*token);
        builder.mint(*mint);
        builder.payer(*self.payer());
        let owner = get_associated_token_address(self.payer(), mint);
        builder.token_owner(owner);

        vec![
            create_associated_token_account(self.payer(), to, mint, &spl_token::id()),
            builder.instruction(),
        ]
    }
}
