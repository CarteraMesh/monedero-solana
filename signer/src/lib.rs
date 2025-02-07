use async_trait::async_trait;
use solana_pubkey::Pubkey;
pub use solana_sdk::signer::SignerError;
use solana_sdk::transaction::VersionedTransaction;
pub use solana_signature::Signature;

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
