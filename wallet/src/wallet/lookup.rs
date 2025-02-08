use {
    super::SolanaWallet,
    monedero_signer_sender::TransactionSignerSender,
    solana_account::Account,
    solana_program::address_lookup_table::program::ID,
    solana_pubkey::Pubkey,
    solana_signature::Signature,
    wasm_client_solana::{
        rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
        solana_account_decoder::UiAccountEncoding,
        RpcAccountInfoConfig,
        RpcProgramAccountsConfig,
    },
};

impl<S: TransactionSignerSender + Send> SolanaWallet<S> {
    #[tracing::instrument(level = "info", skip(accounts))]
    pub async fn lookup_extend(
        &self,
        lookup: Pubkey,
        accounts: Vec<Pubkey>,
    ) -> crate::Result<Signature> {
        let inst = self.instructor.lookup_table_extend(lookup, accounts);
        let sig = self.send_instructions(&[inst], None).await?;
        Ok(sig)
    }

    #[tracing::instrument(level = "info")]
    pub async fn lookup_create(&self) -> crate::Result<(Pubkey, Signature)> {
        let (inst, account) = self.instructor.lookup_table_create().await?;
        let sig = self.send_instructions(&[inst], None).await?;
        Ok((account, sig))
    }

    pub async fn lookup_tables(&self) -> crate::Result<Vec<(Pubkey, Account)>> {
        let rpc = self.rpc();
        let memcmp = Memcmp::new(1, MemcmpEncodedBytes::Base58(self.payer.to_string()));
        let filters = RpcFilterType::Memcmp(memcmp);
        let config = RpcProgramAccountsConfig {
            filters: Some(vec![filters]),
            account_config: RpcAccountInfoConfig::builder()
                .encoding(UiAccountEncoding::Base58)
                .build(),
            with_context: None,
        };
        let result = rpc.get_program_accounts_with_config(&ID, config).await?;
        Ok(result)
    }
}
