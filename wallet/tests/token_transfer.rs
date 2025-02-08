mod setup;
use {
    monedero_solana::{Signature, VersionedTransaction},
    setup::{config, TestConfig},
    solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair, signer::Signer},
    wasm_client_solana::{SolanaRpcClient, VersionedTransactionExtension},
};

#[allow(clippy::missing_panics_doc)]
pub async fn create_account(
    rpc: &SolanaRpcClient,
    new_account: &Keypair,
    payer: &Keypair,
) -> anyhow::Result<Signature> {
    let space: usize = 0;
    let lamports = rpc.get_minimum_balance_for_rent_exemption(space).await?;
    let instruction = solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &new_account.pubkey(),
        lamports,
        space.try_into().expect("no size fits all"),
        &solana_sdk::system_program::id(),
    );
    let tx =
        solana_sdk::transaction::Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    let block = rpc.get_latest_blockhash().await?;
    let mut versioned: VersionedTransaction = tx.into();
    versioned.try_sign(&[payer, new_account], Some(block))?;
    Ok(rpc
        .send_and_confirm_transaction_with_commitment(&versioned, CommitmentConfig::confirmed())
        .await?)
}

#[tokio::test]
#[rstest::rstest]
async fn token_transfer(#[future] config: TestConfig) -> anyhow::Result<()> {
    let config = config.await;
    let tokens = config.tokens_non_empty().await?;
    let token = tokens.first().expect("no tokens for you!");
    TestConfig::explorer(config.wallet.token_transfer(token, &setup::TO, 1).await?);
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn token_transfer_new_account(#[future] config: TestConfig) -> anyhow::Result<()> {
    let config = config.await;
    let kp = Keypair::new();
    let payer = Keypair::from_bytes(&test_utils::KEYPAIR)?;
    TestConfig::explorer(
        create_account(&config.rpc, &kp, &payer)
            .await
            .expect("failed to create account"),
    );
    let tokens = config.tokens_non_empty().await?;
    let token = tokens.first().expect("no tokens for you!");
    TestConfig::explorer(config.wallet.token_transfer(token, &kp.pubkey(), 1).await?);
    Ok(())
}
