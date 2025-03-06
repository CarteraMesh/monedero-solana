use {
    crate::TransactionSignerSender,
    async_trait::async_trait,
    solana_pubkey::Pubkey,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        signer::{keypair::Keypair, Signer, SignerError},
        transaction::VersionedTransaction,
    },
    solana_signature::Signature,
    wasm_client_solana::{SolanaRpcClient, VersionedTransactionExtension},
};

/// KeypairSender is used for testing.
pub struct KeypairSender {
    pk: Pubkey,
    kp: Vec<u8>,
    rpc: SolanaRpcClient,
}

impl KeypairSender {
    pub fn new(kp: Vec<u8>, rpc: &SolanaRpcClient) -> Result<Self, SignerError> {
        let pk = Keypair::from_bytes(&kp)
            .map_err(|e| SignerError::Custom(e.to_string()))?
            .pubkey();
        Ok(Self {
            pk,
            kp,
            rpc: rpc.clone(),
        })
    }
}

#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
impl TransactionSignerSender for KeypairSender {
    fn pubkey(&self) -> Pubkey {
        self.pk
    }

    #[allow(clippy::missing_panics_doc)]
    async fn sign_and_send(
        &self,
        tx: &mut VersionedTransaction,
    ) -> std::result::Result<Signature, SignerError> {
        let kp = Keypair::from_bytes(&self.kp).expect("should never happen");
        tx.try_sign(&[&kp], None)?;
        drop(kp); // kp is not Send
        self.rpc
            .send_and_confirm_transaction_with_commitment(tx, CommitmentConfig::confirmed())
            .await
            .map_err(|e| SignerError::Custom(format!("{e:#?}")))
    }
}
