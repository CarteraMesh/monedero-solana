mod error;
mod jup_ag;
pub use {error::Error, jup_ag::*};
use {solana_pubkey::Pubkey, solana_sdk::instruction::Instruction};
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct JupiterInstructor {
    owner: Pubkey,
}

impl JupiterInstructor {
    pub fn new(owner: &Pubkey) -> Self {
        Self { owner: *owner }
    }

    /// See https://station.jup.ag/docs/old/apis/swap-api#instructions-instead-of-transaction
    pub async fn swap(
        &self,
        from: &Pubkey,
        to: &Pubkey,
        amount: u64,
        quote: QuoteConfig,
        wrap_and_unwrap_sol: bool,
    ) -> Result<(Vec<Instruction>, Vec<Pubkey>)> {
        let quotes = jup_ag::quote(*from, *to, amount, quote).await?;
        let mut request: SwapRequest = SwapRequest::new(self.owner, quotes);
        request.wrap_and_unwrap_sol = Some(wrap_and_unwrap_sol);
        let mut swap_instructions = jup_ag::swap_instructions(request).await?;

        let mut instructions = Vec::with_capacity(
            swap_instructions.compute_budget_instructions.len()
                + swap_instructions.setup_instructions.len()
                + 6,
        );
        instructions.append(&mut swap_instructions.compute_budget_instructions);
        instructions.append(&mut swap_instructions.setup_instructions);
        if let Some(i) = swap_instructions.token_ledger_instruction.take() {
            instructions.push(i);
        }
        instructions.push(swap_instructions.swap_instruction);
        if let Some(cleanup) = swap_instructions.cleanup_instruction.take() {
            instructions.push(cleanup);
        }
        Ok((
            instructions,
            swap_instructions.address_lookup_table_addresses,
        ))
    }
}

#[cfg(test)]
mod test {
    use {
        crate::{jup_ag::QuoteConfig, JupiterInstructor},
        solana_pubkey::Pubkey,
        solana_sdk::native_token::sol_to_lamports,
        spl_token::native_mint::id,
        std::str::FromStr,
        test_utils::setup,
    };

    #[tokio::test]
    async fn swap_wsol() -> anyhow::Result<()> {
        setup();
        let usdc = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")?;
        let jup = JupiterInstructor::new(&test_utils::OWNER);
        let quote = QuoteConfig {
            slippage_bps: Some(1),
            ..Default::default()
        };
        let (instructions, lookups) = jup
            .swap(&id(), &usdc, sol_to_lamports(1.0), quote, true)
            .await?;
        assert!(!instructions.is_empty());
        assert!(!lookups.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn swap_sol() -> anyhow::Result<()> {
        setup();
        let usdc = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")?;
        let jup = JupiterInstructor::new(&test_utils::OWNER);
        let quote = QuoteConfig {
            slippage_bps: Some(1),
            ..Default::default()
        };
        let (instructions, lookups) = jup
            .swap(&id(), &usdc, sol_to_lamports(1.0), quote, false)
            .await?;
        assert!(!instructions.is_empty());
        assert!(!lookups.is_empty());
        Ok(())
    }
}
