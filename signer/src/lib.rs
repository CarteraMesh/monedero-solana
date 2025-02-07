use {
    async_trait::async_trait,
    solana_pubkey::Pubkey,
    solana_sdk::transaction::VersionedTransaction,
};
pub use {solana_sdk::signer::SignerError, solana_signature::Signature};

mod kp;

pub use kp::KeyPairSender;

#[async_trait(?Send)]
pub trait TransactionSignerSender {
    fn pubkey(&self) -> Pubkey;
    async fn sign_and_send(
        &self,
        tx: &mut VersionedTransaction,
    ) -> std::result::Result<Signature, SignerError>;
}
