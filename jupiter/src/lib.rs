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

    pub async fn quote(
        &self,
        from: &Pubkey,
        to: &Pubkey,
        amount: u64,
        quote: QuoteConfig,
    ) -> Result<Quote> {
        jup_ag::quote(*from, *to, amount, quote).await
    }

    pub async fn swap_request(swap: SwapRequest) -> Result<(Vec<Instruction>, Vec<Pubkey>)> {
        let mut swap_instructions = jup_ag::swap_instructions(swap).await?;
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

    /// See https://station.jup.ag/docs/old/apis/swap-api#instructions-instead-of-transaction
    pub async fn swap(
        &self,
        from: &Pubkey,
        to: &Pubkey,
        amount: u64,
        quote: QuoteConfig,
        wrap_and_unwrap_sol: bool,
    ) -> Result<(Vec<Instruction>, Vec<Pubkey>)> {
        let quotes = self.quote(from, to, amount, quote).await?;
        let mut request = SwapRequest::new(self.owner, quotes);
        request.wrap_and_unwrap_sol = Some(wrap_and_unwrap_sol);
        request.prioritization_fee_lamports = PrioritizationFeeLamports::Auto;
        Self::swap_request(request).await
    }

    pub async fn price(from: &Pubkey, to: &Pubkey, ui_amount: f64) -> Result<Price> {
        jup_ag::price(*from, *to, ui_amount).await
    }
}

#[cfg(test)]
mod test {
    use {
        crate::{
            jup_ag::{self, QuoteConfig},
            JupiterInstructor,
            PrioritizationFeeLamports,
            PriorityLevel,
            SwapMode,
            SwapRequest,
        },
        solana_pubkey::Pubkey,
        solana_sdk::native_token::sol_to_lamports,
        spl_token::native_mint::id,
        std::str::FromStr,
        test_utils::setup,
    };
    const USDC: Pubkey = Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

    #[tokio::test]
    async fn swap_request_serde() -> anyhow::Result<()> {
        setup();
        let jup = JupiterInstructor::new(&test_utils::OWNER);
        let quote_config = QuoteConfig {
            slippage_bps: Some(1),
            ..Default::default()
        };
        let quote = jup
            .quote(&id(), &USDC, sol_to_lamports(1.1), quote_config)
            .await?;
        let mut req = SwapRequest::new(test_utils::OWNER, quote);
        req.prioritization_fee_lamports = PrioritizationFeeLamports::Exact { lamports: 2 };
        serde_json::to_string(&req)?;
        req.prioritization_fee_lamports =
            PrioritizationFeeLamports::AutoMultiplier { multiplier: 1 };
        serde_json::to_string(&req)?;

        req.prioritization_fee_lamports =
            PrioritizationFeeLamports::JitoTipLamports { lamports: 100 };
        serde_json::to_string(&req)?;
        req.prioritization_fee_lamports = PrioritizationFeeLamports::PriorityLevelWithMaxLamports {
            priority_level: PriorityLevel::High,
            max_lamports: 12321,
        };
        serde_json::to_string(&req)?;

        let swap_mode: SwapMode = SwapMode::from_str("ExactIn")?;
        assert_eq!("ExactIn", format!("{swap_mode}"));
        let swap_mode: SwapMode = SwapMode::from_str("ExactOut")?;
        assert_eq!("ExactOut", format!("{swap_mode}"));

        Ok(())
    }

    #[tokio::test]
    async fn price() -> anyhow::Result<()> {
        setup();
        JupiterInstructor::price(&id(), &USDC, 1.1).await?;
        Ok(())
    }

    #[tokio::test]
    async fn tokens() -> anyhow::Result<()> {
        jup_ag::tokens().await?;
        Ok(())
    }

    #[tokio::test]
    async fn swap_wsol() -> anyhow::Result<()> {
        setup();
        let jup = JupiterInstructor::new(&test_utils::OWNER);
        let quote = QuoteConfig {
            slippage_bps: Some(1),
            ..Default::default()
        };
        let (instructions, lookups) = jup
            .swap(&id(), &USDC, sol_to_lamports(1.0), quote, true)
            .await?;
        assert!(!instructions.is_empty());
        assert!(!lookups.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn swap_sol() -> anyhow::Result<()> {
        setup();
        let jup = JupiterInstructor::new(&test_utils::OWNER);
        let quote = QuoteConfig {
            slippage_bps: Some(1),
            ..Default::default()
        };
        let (instructions, lookups) = jup
            .swap(&id(), &USDC, sol_to_lamports(1.0), quote, false)
            .await?;
        assert!(!instructions.is_empty());
        assert!(!lookups.is_empty());
        Ok(())
    }
}
