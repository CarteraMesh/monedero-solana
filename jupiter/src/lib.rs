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
        // let request = SwapRequest::new(self.owner, quotes);
        let request: SwapRequest = SwapRequest::builder()
            .quote_response(quotes)
            .user_public_key(self.owner)
            .wrap_and_unwrap_sol(wrap_and_unwrap_sol)
            .prioritization_fee_lamports(PrioritizationFeeLamports::Auto)
            .build();
        Self::swap_request(request).await
    }
}

#[cfg(test)]
mod test {
    use {
        crate::{
            jup_ag::QuoteConfig, JupiterInstructor, PrioritizationFeeLamports, Quote, SwapMode,
            SwapRequest,
        },
        solana_pubkey::Pubkey,
        solana_sdk::native_token::sol_to_lamports,
        spl_token::native_mint::id,
        test_utils::setup,
    };
    const USDC: Pubkey = Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

    // #[test]
    // fn swap_request() {
    //     let quote = Quote::builder()
    //         .input_mint(USDC)
    //         .in_amount(1)
    //         .slippage_bps(0)
    //         .out_amount(1)
    //         .output_mint(USDC)
    //         .route_plan(vec![])
    //         .price_impact_pct(0.0)
    //         .other_amount_threshold(1)
    //         .swap_mode(SwapMode::ExactIn.to_string())
    //         .build();
    //     let _ = SwapRequest::builder()
    //         .quote_response(quote)
    //         .prioritization_fee_lamports(PrioritizationFeeLamports::Auto)
    //         .wrap_and_unwrap_sol(true)
    //         .user_public_key(&TESTNET::OWNER)
    //         .build();
    // }
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
