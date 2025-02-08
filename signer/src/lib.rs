use {
    async_trait::async_trait,
    solana_pubkey::Pubkey,
    solana_sdk::transaction::VersionedTransaction,
};
pub use {solana_sdk::signer::SignerError, solana_signature::Signature};

mod kp;

pub use kp::KeypairSender;

// #[cfg(target_family = "wasm")]
// #[async_trait(?Send)]
// pub trait TransactionSignerSender {
// fn pubkey(&self) -> Pubkey;
// async fn sign_and_send(
// &self,
// tx: &mut VersionedTransaction,
// ) -> std::result::Result<Signature, SignerError>;
// }

#[cfg(not(target_family = "wasm"))]
#[async_trait]
pub trait TransactionSignerSender: Sync {
    fn pubkey(&self) -> Pubkey;
    async fn sign_and_send(
        &self,
        tx: &mut VersionedTransaction,
    ) -> std::result::Result<Signature, SignerError>;
}

#[cfg(test)]
mod test {

    use {super::*, solana_sdk::signature::Keypair, wasm_client_solana::DEVNET};

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
