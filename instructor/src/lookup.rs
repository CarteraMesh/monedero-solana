use {
    crate::{Instructor, Result},
    solana_pubkey::Pubkey,
    solana_sdk::{
        account::from_account,
        clock::Clock,
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        sysvar,
    },
};

impl Instructor {
    pub async fn lookup_table_create(&self) -> Result<(Instruction, Pubkey)> {
        let clock = self
            .rpc()
            .get_account_with_commitment(&sysvar::clock::id(), CommitmentConfig::finalized())
            .await?;

        let clock = clock.ok_or_else(|| crate::Error::ClockAccountNotFound(sysvar::clock::id()))?;
        let clock_account: Clock = from_account(&clock).ok_or(crate::Error::InvalidClockAccount)?;
        let payer = *self.payer();
        let (instruction, account) =
            solana_sdk::address_lookup_table::instruction::create_lookup_table(
                payer,
                payer,
                clock_account.slot,
            );
        Ok((instruction, account))
    }

    pub fn lookup_table_extend(
        &self,
        lookup_table_address: Pubkey,
        accounts: Vec<Pubkey>,
    ) -> Instruction {
        let payer = *self.payer();
        solana_sdk::address_lookup_table::instruction::extend_lookup_table(
            lookup_table_address,
            payer,
            Some(payer),
            accounts,
        )
    }

    pub fn lookup_table_deactivate(&self, lookup_table_address: &Pubkey) -> Instruction {
        let payer = *self.payer();
        solana_sdk::address_lookup_table::instruction::deactivate_lookup_table(
            *lookup_table_address,
            payer,
        )
    }

    pub fn lookup_table_close(&self, lookup_table_address: &Pubkey) -> Instruction {
        let payer = *self.payer();
        solana_sdk::address_lookup_table::instruction::close_lookup_table(
            *lookup_table_address,
            payer,
            payer,
        )
    }
}
