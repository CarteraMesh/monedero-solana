use {
    base64::{prelude::BASE64_STANDARD, Engine},
    monedero_signer_sender::TransactionSignerSender,
    monedero_solana_instructor::{token::TokenClient, Instructor},
    solana_pubkey::Pubkey,
    solana_sdk::{
        address_lookup_table::AddressLookupTableAccount,
        instruction::Instruction,
        signature::Signature,
        signers::Signers,
        transaction::VersionedTransaction,
    },
    std::fmt::{Debug, Display},
    tracing::Level,
    wasm_client_solana::{
        solana_account_decoder::parse_token::UiTokenAccount,
        SimulateTransactionResponseValue,
        SolanaRpcClient as RpcClient,
        VersionedTransactionExtension,
    },
};

// mod name;
// mod sweep;
mod lookup;
mod native;
mod stake;
mod swap;
mod tokens;

#[derive(Debug)]
pub enum FeeType {
    Units(u32),
    Priority(u64),
}

#[derive(Clone)]
pub struct SolanaWallet<S: TransactionSignerSender> {
    signer: S,
    instructor: Instructor,
    tc: TokenClient,
    payer: Pubkey,
    rpc: RpcClient,
    default_lookup: Vec<AddressLookupTableAccount>,
}

impl<S: TransactionSignerSender> Debug for SolanaWallet<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.pk())
    }
}

impl<S: TransactionSignerSender> Display for SolanaWallet<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.pk())
    }
}

impl<S: TransactionSignerSender> SolanaWallet<S> {
    pub fn new(signer: S, rpc: &RpcClient) -> Self {
        let payer = signer.pubkey();
        let instructor = Instructor::new(&payer, rpc);
        // let signer = Arc::new(signer);
        let tc = instructor.token_client().clone();
        Self {
            signer,
            instructor,
            tc,
            payer,
            rpc: rpc.clone(),
            default_lookup: Vec::new(),
        }
    }

    pub async fn with_lookup(
        signer: S,
        rpc: &RpcClient,
        lookup_address: &Pubkey,
    ) -> crate::Result<Self> {
        let mut wallet = Self::new(signer, rpc);
        let result = rpc.get_address_lookup_table(lookup_address).await?;
        let lookup = result
            .optional_address_lookup_table_account(lookup_address)
            .ok_or(crate::Error::LookupTableNotFound(*lookup_address))?;
        wallet.default_lookup = vec![lookup];
        Ok(wallet)
    }

    #[tracing::instrument(level = "info", skip(addresses))]
    async fn lookup(&self, addresses: &[Pubkey]) -> crate::Result<Vec<AddressLookupTableAccount>> {
        let mut lookups = Vec::with_capacity(3);
        for a in addresses {
            let addr_table = self.rpc.get_address_lookup_table(a).await?;
            if let Some(t) = addr_table.optional_address_lookup_table_account(a) {
                lookups.push(t);
            }
        }
        Ok(lookups)
    }

    pub async fn simulate(
        &self,
        tx: &VersionedTransaction,
    ) -> crate::Result<SimulateTransactionResponseValue> {
        let span = tracing::span!(
            Level::INFO,
            "simulate",
            wallet = format!("{self}"),
            computeUnits = 0,
        );
        let _ctx = span.enter();
        let r = self.rpc.simulate_transaction(tx).await?.value;
        tracing::debug!("simulation result {:#?}", r);

        if let Some(ref e) = r.err {
            return Err(crate::Error::SimulateError(format!("{e:#?}")));
        }

        r.units_consumed.inspect(|u| {
            span.record("computeUnits", u);
        });
        Ok(r)
    }

    pub async fn memo(&self, message: &str) -> crate::Result<Signature> {
        let ix = vec![self.instructor.memo(message)];
        self.send_instructions(&ix, None).await
    }

    #[allow(clippy::future_not_send)]
    pub async fn send_instructions_with_signer<T>(
        &self,
        ix: &[Instruction],
        signers: &T,
    ) -> crate::Result<Signature>
    where
        T: Signers,
    {
        let block = self.rpc.get_latest_blockhash().await?;
        let mut tx = VersionedTransaction::new_unsigned_v0(&self.payer, ix, &[], block)?;
        tx.try_sign(signers, None)?;
        let encoded = BASE64_STANDARD.encode(bincode::serialize(&tx)?);
        tracing::info!("encoded {encoded}");
        let sig = self.signer.sign_and_send(&mut tx).await?;
        Ok(sig)
    }

    pub async fn send_instructions(
        &self,
        ix: &[Instruction],
        lookups: Option<Vec<Pubkey>>,
    ) -> crate::Result<Signature> {
        let block = self.rpc.get_latest_blockhash().await?;
        let mut tx: VersionedTransaction = match lookups {
            None => {
                VersionedTransaction::new_unsigned_v0(&self.payer, ix, &self.default_lookup, block)?
            }
            Some(ref addresses) => {
                let table = self.lookup(addresses).await?;
                VersionedTransaction::new_unsigned_v0(&self.payer, ix, &table, block)?
            }
        };
        let encoded = BASE64_STANDARD.encode(bincode::serialize(&tx)?);
        tracing::debug!("encoded {encoded}");
        let _ = self.simulate(&tx).await?;
        let sig = self.signer.sign_and_send(&mut tx).await?;
        Ok(sig)
    }

    pub fn rpc(&self) -> &RpcClient {
        &self.rpc
    }

    pub fn pk(&self) -> &Pubkey {
        &self.payer
    }

    pub async fn balance(&self) -> crate::Result<u64> {
        Ok(self.rpc.get_balance(&self.payer).await?)
    }

    #[tracing::instrument(level = "info")]
    pub async fn transfer_memo(
        &self,
        to: &Pubkey,
        lamports: u64,
        memo: &str,
    ) -> crate::Result<Signature> {
        let inst = vec![
            self.instructor.memo(memo),
            solana_sdk::system_instruction::transfer(&self.payer, to, lamports),
        ];
        self.send_instructions(&inst, None).await
    }

    #[tracing::instrument(level = "info")]
    pub async fn transfer(&self, to: &Pubkey, lamports: u64) -> crate::Result<Signature> {
        let inst = vec![solana_sdk::system_instruction::transfer(
            &self.payer,
            to,
            lamports,
        )];
        self.send_instructions(&inst, None).await
    }

    pub async fn get_token(&self, mint: &Pubkey) -> crate::Result<Option<UiTokenAccount>> {
        Ok(self.tc.get_token(mint).await?)
    }

    // pub async fn fees(&self) -> crate::Result<Vec<FeeType>> {
    //    let mut fees: Vec<FeeType> = Vec::with_capacity(10);
    //    let to = Pubkey::new_unique();
    //    let transfer_ix = self.transfer_instructions(&to, 100);
    //    let fee = self
    //        .fee_service
    //        .simulate(&transfer_ix)
    //        .await?
    //        .unwrap_or_default();
    //    fees.push(FeeType::Units(fee));
    //    let fee = self
    //        .fee_service
    //        .compute_fee()
    //        .await
    //        .ok()
    //        .unwrap_or_default();
    //    fees.push(FeeType::Priority(fee));
    //    Ok(fees)
    //}
    //
}
