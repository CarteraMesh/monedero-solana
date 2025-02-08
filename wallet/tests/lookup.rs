mod setup;
use {
    monedero_solana::{sol_to_lamports, RpcClient, Signature, SolanaWallet, VersionedTransaction},
    setup::{config, TestConfig},
    solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair, signer::Signer},
    std::time::Duration,
    wasm_client_solana::VersionedTransactionExtension,
};

#[allow(clippy::missing_panics_doc)]
pub async fn create_account(
    rpc: &RpcClient,
    new_account: &Keypair,
    payer: &Keypair,
) -> anyhow::Result<Signature> {
    let space: usize = 0;
    let lamports = rpc.get_minimum_balance_for_rent_exemption(space).await?;
    let mut instruction = vec![solana_sdk::system_instruction::create_account(
        &payer.pubkey(),
        &new_account.pubkey(),
        lamports,
        space.try_into().expect("no size fits all"),
        &solana_sdk::system_program::id(),
    )];
    instruction.push(solana_sdk::system_instruction::transfer(
        &payer.pubkey(),
        &new_account.pubkey(),
        sol_to_lamports(0.01),
    ));
    let tx =
        solana_sdk::transaction::Transaction::new_with_payer(&instruction, Some(&payer.pubkey()));
    let block = rpc.get_latest_blockhash().await?;
    let mut versioned: VersionedTransaction = tx.into();
    versioned.try_sign(&[payer, new_account], Some(block))?;
    Ok(rpc
        .send_and_confirm_transaction_with_commitment(&versioned, CommitmentConfig::confirmed())
        .await?)
}

#[tokio::test]
#[rstest::rstest]
async fn lookup_list(#[future] config: TestConfig) -> anyhow::Result<()> {
    let config = config.await;
    let wallet = &config.wallet;
    let lookups = wallet.lookup_tables().await?;
    assert!(!lookups.is_empty());
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn lookup_tests(#[future] config: TestConfig) -> anyhow::Result<()> {
    let config = config.await;
    let rpc = &config.rpc;
    let kp = Keypair::new();
    let payer = Keypair::from_bytes(&test_utils::KEYPAIR)?;
    create_account(rpc, &kp, &payer).await?;
    tracing::info!("new solana wallet created {}", kp.pubkey());
    rpc.wait_for_new_block(1).await?;
    tokio::time::sleep(Duration::from_secs(30)).await;
    let new_wallet = SolanaWallet::new(test_utils::keypair_sender(Some(rpc.clone())), rpc);
    let (account, sig) = new_wallet.lookup_create().await?;
    tracing::info!("lookup table created: {}", account);
    TestConfig::explorer(sig);

    new_wallet
        .lookup_extend(account, vec![payer.pubkey(), spl_token::id()])
        .await?;

    let lookups = new_wallet.lookup_tables().await?;
    assert!(!lookups.is_empty());
    Ok(())
}
