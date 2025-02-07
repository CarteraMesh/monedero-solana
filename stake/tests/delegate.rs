mod setup;

use {
    setup::{config, TestConfig},
    solana_sdk::{native_token::sol_to_lamports, signature::Keypair, signer::Signer},
};

#[tokio::test]
#[rstest::rstest]
async fn create_account(config: TestConfig) -> anyhow::Result<()> {
    let c = &config.client;
    let (_, inst) = c
        .create_account("random seed", sol_to_lamports(2.1))
        .await?;
    assert_eq!(2, inst.len());
    assert_eq!(inst[0].program_id, solana_sdk::system_program::id());
    assert_eq!(inst[1].program_id, solana_sdk::stake::program::id());
    Ok(())
}

#[tokio::test]
#[rstest::rstest]
async fn create_delegate(config: TestConfig) -> anyhow::Result<()> {
    let c = &config.client;
    let validators = c.validators().await?;
    let vote_account = validators.first().expect("no vote/validators");
    let stake_account = Keypair::new();
    let inst = c
        .create_delegate(
            &stake_account.pubkey(),
            &vote_account.vote_pubkey,
            sol_to_lamports(2.1),
        )
        .await?;
    assert_eq!(3, inst.len());
    assert_eq!(inst[0].program_id, solana_sdk::system_program::id());
    assert_eq!(inst[1].program_id, solana_sdk::stake::program::id());
    assert_eq!(inst[2].program_id, solana_sdk::stake::program::id());
    Ok(())
}
