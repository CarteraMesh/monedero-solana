#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    QuoteError(#[from] jup_ag::Error),
}
