mod setup;
use {
    monedero_solana::{sol_to_lamports, RpcClient, Signature, VersionedTransaction},
    setup::{config, TestConfig},
    solana_sdk::{
        address_lookup_table::state::LookupTableStatus,
        commitment_config::CommitmentConfig,
        signature::Keypair,
        signer::Signer,
        sysvar::slot_hashes::{id as slot_hashes_id, SlotHashes},
    },
    wasm_client_solana::{SolanaRpcClient, VersionedTransactionExtension},
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
    // for (address, state) in lookups {
    //    tracing::info!("{address} {state:#?}");
    //}
    Ok(())
}

async fn slot_hashes(rpc: &SolanaRpcClient) -> anyhow::Result<(u64, SlotHashes)> {
    let slot = rpc.get_slot().await?;
    let account_data = rpc.get_account_data(&slot_hashes_id()).await?;
    let slot_hashes: SlotHashes = bincode::deserialize(&account_data)?;

    Ok((slot, slot_hashes))
}

#[tokio::test]
#[rstest::rstest]
async fn lookup_close(#[future] config: TestConfig) -> anyhow::Result<()> {
    let config = config.await;
    let wallet = &config.wallet;
    let lookups = wallet.lookup_tables().await?;
    let rpc = &config.rpc;
    let (slot, slot_hashes) = slot_hashes(rpc).await?;
    assert!(!lookups.is_empty());
    let to_close = lookups.into_iter().find(|t| {
        t.0 != setup::LOOKUP
            && t.1.meta.status(slot, &slot_hashes) == LookupTableStatus::Deactivated
    });
    if to_close.is_none() {
        tracing::warn!("no lookup tables to close");
        return Ok(());
    }
    let to_close = to_close.unwrap().0;
    wallet.lookup_close(&to_close).await?;
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn lookup_deactivate(#[future] config: TestConfig) -> anyhow::Result<()> {
    let config = config.await;
    let wallet = &config.wallet;
    let lookups = wallet.lookup_tables().await?;
    let rpc = &config.rpc;
    let (slot, slot_hashes) = slot_hashes(rpc).await?;
    assert!(!lookups.is_empty());
    let to_deactivate = lookups.into_iter().find(|t| {
        t.0 != setup::LOOKUP && t.1.meta.status(slot, &slot_hashes) == LookupTableStatus::Activated
    });
    if to_deactivate.is_none() {
        tracing::warn!("no lookup tables to deactivate");
        return Ok(());
    }
    let (address, _state) = to_deactivate.unwrap();
    let sig = wallet.lookup_deactivate(&address).await?;
    TestConfig::explorer(sig);
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn lookup_create(#[future] config: TestConfig) -> anyhow::Result<()> {
    let config = config.await;
    let wallet = &config.wallet;
    let (account, sig) = wallet.lookup_create().await?;
    tracing::info!("lookup table created: {}", account);
    TestConfig::explorer(sig);
    // Unstable
    // rpc.wait_for_new_block(4).await?;
    // tokio::time::sleep(Duration::from_secs(30)).await;
    // wallet.lookup_extend(account, vec![spl_token::id()]).await?;
    Ok(())
}
