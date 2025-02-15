use {
    super::SolanaWallet,
    monedero_signer_sender::TransactionSignerSender,
    solana_program::address_lookup_table::program::ID,
    solana_pubkey::Pubkey,
    solana_sdk::address_lookup_table::state::AddressLookupTable,
    solana_signature::Signature,
    std::borrow::Cow,
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

    #[tracing::instrument(level = "info")]
    pub async fn lookup_deactivate(&self, address: &Pubkey) -> crate::Result<Signature> {
        let inst = self.instructor.lookup_table_deactivate(address);
        let sig = self.send_instructions(&[inst], None).await?;
        Ok(sig)
    }

    #[tracing::instrument(level = "info")]
    pub async fn lookup_close(&self, address: &Pubkey) -> crate::Result<Signature> {
        let inst = self.instructor.lookup_table_close(address);
        let sig = self.send_instructions(&[inst], None).await?;
        Ok(sig)
    }

    /// https://solana.stackexchange.com/questions/6539/how-to-get-address-lookup-tables-owned-by-given-pubkey-via-filtering-the-lookupt
    pub async fn lookup_tables(&self) -> crate::Result<Vec<(Pubkey, AddressLookupTable)>> {
        let rpc = self.rpc();
        let memcmp = Memcmp::new(22, MemcmpEncodedBytes::Base58(self.payer.to_string()));
        let filters = RpcFilterType::Memcmp(memcmp);
        let config = RpcProgramAccountsConfig {
            filters: Some(vec![filters]),
            account_config: RpcAccountInfoConfig::builder()
                .encoding(UiAccountEncoding::Base64)
                .build(),
            with_context: None,
        };
        let result = rpc.get_program_accounts_with_config(&ID, config).await?;
        let mut decoded = Vec::with_capacity(result.len());
        for (address, account) in result {
            // Deserialize using the account's data slice.
            let table_state_borrowed = AddressLookupTable::deserialize(&account.data)
                .map_err(|e| crate::Error::LookupTableDecodeError(e.to_string()))?;

            // Convert to an owned version so that the result doesn't borrow from
            // `account.data`.
            let table_state_owned = AddressLookupTable {
                meta: table_state_borrowed.meta,
                addresses: Cow::Owned(table_state_borrowed.addresses.into_owned()),
            };
            decoded.push((address, table_state_owned));
        }
        Ok(decoded)
    }
}
