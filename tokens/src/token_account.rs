use {
    crate::solana_account_decoder::parse_token::UiTokenAccount,
    serde::{Deserialize, Serialize},
    serde_with::{serde_as, DisplayFromStr},
    solana_pubkey::Pubkey,
    std::{
        cmp::Ordering,
        collections::BTreeSet,
        fmt::{Debug, Display, Formatter},
    },
};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
}

impl Display for TokenMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.symbol)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnsupportedAccount {
    pub address: String,
    pub err: String,
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct TokenAccount {
    #[serde_as(as = "DisplayFromStr")]
    pub address: Pubkey,
    #[serde_as(as = "DisplayFromStr")]
    pub program_id: Pubkey,
    pub is_associated: bool,
    pub account: UiTokenAccount,
    pub has_permanent_delegate: bool,
    pub metadata: TokenMetadata,
}

impl TokenAccount {
    pub fn amount(&self) -> u64 {
        self.account
            .token_amount
            .amount
            .parse::<u64>()
            .ok()
            .unwrap_or_default()
    }

    fn fmt_common(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "amt={},address={},token={},legacy={}",
            self.amount(),
            self.address,
            self.account.mint,
            !spl_token_2022::check_id(&self.program_id)
        )
    }
}

impl Debug for TokenAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_common(f)
    }
}
impl Display for TokenAccount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_common(f)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAccounts {
    pub accounts: BTreeSet<TokenAccount>,
    pub unsupported_accounts: Vec<UnsupportedAccount>,
    pub max_len_balance: usize,
    pub aux_len: usize,
    pub explicit_token: bool,
}

impl PartialEq<Self> for TokenAccount {
    fn eq(&self, other: &Self) -> bool {
        if self.metadata.symbol.eq(&self.metadata.symbol) {
            return self.address == other.address;
        }
        self.metadata.symbol.eq(&other.metadata.symbol)
    }
}

impl PartialOrd<Self> for TokenAccount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TokenAccount {}

impl Ord for TokenAccount {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.metadata.symbol.eq(&self.metadata.symbol) {
            return self.address.cmp(&other.address);
        }
        self.metadata.symbol.cmp(&other.metadata.symbol)
    }
}

impl Display for TokenAccounts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "accounts:{} unsupported:{} max_len_balance:{} aux_len:{}",
            self.accounts.len(),
            self.unsupported_accounts.len(),
            self.max_len_balance,
            self.aux_len
        )
    }
}
