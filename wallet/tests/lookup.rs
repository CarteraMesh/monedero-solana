mod setup;
use setup::{config, TestConfig};

#[tokio::test]
#[rstest::rstest]
async fn lookup_list(#[future] config: TestConfig) -> anyhow::Result<()> {
    let config = config.await;
    let wallet = &config.wallet;
    let lookups = wallet.lookup_tables().await?;
    for (address, state) in lookups {
        tracing::info!("{address} {state:#?}");
    }
    Ok(())
}

// async fn slot_hashes(rpc: &SolanaRpcClient) -> anyhow::Result<(u64,
// SlotHashes)> {     let slot = rpc.get_slot().await?;
//     let account_data = rpc.get_account_data(&slot_hashes_id()).await?;
//     let slot_hashes: SlotHashes = bincode::deserialize(&account_data)?;
//     Ok((slot, slot_hashes))
// }
//
// #[tokio::test]
// #[rstest::rstest]
// async fn lookup_close(#[future] config: TestConfig) -> anyhow::Result<()> {
//     let config = config.await;
//     let wallet = &config.wallet;
//     let lookups = wallet.lookup_tables().await?;
//     let rpc = &config.rpc;
//     let (slot, slot_hashes) = slot_hashes(rpc).await?;
//     assert!(!lookups.is_empty());
//     let to_close = lookups.into_iter().find(|t| {
//         t.0 != setup::LOOKUP
//             && t.1.meta.status(slot, &slot_hashes) ==
// LookupTableStatus::Deactivated     });
//     if to_close.is_none() {
//         tracing::warn!("no lookup tables to close");
//         return Ok(());
//     }
//     let to_close = to_close.unwrap().0;
//     wallet.lookup_close(&to_close).await?;
//     Ok(())
// }
//
// #[tokio::test]
// #[rstest::rstest]
// async fn lookup_deactivate(#[future] config: TestConfig) ->
// anyhow::Result<()> {     let config = config.await;
//     let wallet = &config.wallet;
//     let lookups = wallet.lookup_tables().await?;
//     let rpc = &config.rpc;
//     let (slot, slot_hashes) = slot_hashes(rpc).await?;
//     if lookups.is_empty() {
//         return Ok(());
//     }
//     let to_deactivate = lookups.into_iter().find(|t| {
//         t.0 != setup::LOOKUP && t.1.meta.status(slot, &slot_hashes) ==
// LookupTableStatus::Activated     });
//     if to_deactivate.is_none() {
//         tracing::warn!("no lookup tables to deactivate");
//         return Ok(());
//     }
//     let (address, _state) = to_deactivate.unwrap();
//     let sig = wallet.lookup_deactivate(&address).await?;
//     TestConfig::explorer(sig);
//     Ok(())
// }
//
// #[tokio::test]
// #[rstest::rstest]
// async fn lookup_create(#[future] config: TestConfig) -> anyhow::Result<()> {
//     let config = config.await;
//     let wallet = &config.wallet;
//     let (account, sig) = wallet.lookup_create().await?;
//     tracing::info!("lookup table created: {}", account);
//     TestConfig::explorer(sig);
//     // rpc.wait_for_new_block(4).await?;
//     // tokio::time::sleep(std::time::Duration::from_secs(30)).await;
//     // wallet.lookup_extend(account, vec![spl_token::id()]).await?;
//     Ok(())
// }
