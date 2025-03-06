use {
    async_trait::async_trait,
    solana_pubkey::Pubkey,
    solana_sdk::transaction::VersionedTransaction,
};
pub use {solana_sdk::signer::SignerError, solana_signature::Signature};

mod kp;

pub use kp::KeypairSender;

#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
pub trait TransactionSignerSender {
    fn pubkey(&self) -> Pubkey;
    async fn sign_and_send(
        &self,
        tx: &mut VersionedTransaction,
    ) -> std::result::Result<Signature, SignerError>;
}

pub struct NoopSigner {
    pubkey: Pubkey,
}

impl From<Pubkey> for NoopSigner {
    fn from(pubkey: Pubkey) -> Self {
        Self::new(pubkey)
    }
}

impl NoopSigner {
    pub fn new(pubkey: Pubkey) -> Self {
        Self { pubkey }
    }
}

#[cfg_attr(target_family = "wasm", async_trait(?Send))]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
impl TransactionSignerSender for NoopSigner {
    fn pubkey(&self) -> Pubkey {
        self.pubkey
    }

    async fn sign_and_send(
        &self,
        _tx: &mut VersionedTransaction,
    ) -> std::result::Result<Signature, SignerError> {
        return Err(SignerError::Custom("Not Implemented".to_owned()));
    }
}

#[cfg(test)]
mod test {

    use {
        super::*,
        solana_sdk::{hash::Hash, signature::Keypair, signer::Signer},
        wasm_client_solana::{VersionedTransactionExtension, DEVNET},
    };

    #[tokio::test]
    async fn test_into() -> anyhow::Result<()> {
        let kp = Keypair::new();
        let pk = kp.pubkey();
        let signer: NoopSigner = pk.into();
        assert_eq!(signer.pubkey(), kp.pubkey());

        let inst = vec![solana_sdk::system_instruction::transfer(
            &signer.pubkey(),
            &signer.pubkey(),
            1,
        )];
        let mut tx =
            VersionedTransaction::new_unsigned_v0(&signer.pubkey(), &inst, &[], Hash::default())?;
        let result = signer.sign_and_send(&mut tx).await;
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_bad_key() -> anyhow::Result<()> {
        let kp = Keypair::new();
        let rpc = wasm_client_solana::SolanaRpcClient::new(DEVNET);
        let _ = KeypairSender::new(kp.to_bytes().to_vec(), &rpc)?;
        let should_fail = KeypairSender::new(vec![0, 0, 0, 0], &rpc);
        assert!(should_fail.is_err());
        Ok(())
    }
}
