use {
    crate::token_account::{TokenAccount, TokenAccounts, TokenMetadata, UnsupportedAccount},
    solana_pubkey::Pubkey,
    spl_associated_token_account_client::address::get_associated_token_address_with_program_id,
    std::collections::BTreeSet,
    wasm_client_solana::{
        solana_account_decoder::{parse_token::TokenAccountType, UiAccountData},
        RpcKeyedAccount,
    },
};

pub fn sort_and_parse_token_accounts(
    owner: &Pubkey,
    keyed_accounts: Vec<RpcKeyedAccount>,
    explicit_token: bool,
) -> TokenAccounts {
    let mut accounts: BTreeSet<TokenAccount> = BTreeSet::new();
    let mut unsupported_accounts = vec![];
    let mut max_len_balance = 0;
    let mut aux_count = 0;

    for keyed_account in keyed_accounts {
        let address = keyed_account.pubkey;
        let program_id = keyed_account.account.owner;

        if let UiAccountData::Json(parsed_account) = keyed_account.account.data {
            match serde_json::from_value(parsed_account.parsed) {
                Ok(TokenAccountType::Account(ui_token_account)) => {
                    let mint = ui_token_account.mint;
                    let is_associated =
                        get_associated_token_address_with_program_id(owner, &mint, &program_id)
                            == address;

                    if !is_associated {
                        aux_count += 1;
                    }

                    max_len_balance = max_len_balance.max(
                        ui_token_account
                            .token_amount
                            .real_number_string_trimmed()
                            .len(),
                    );
                    // let metadata = self.metadata_client.get(&ui_token_account);
                    let account = TokenAccount {
                        address,
                        program_id,
                        account: ui_token_account,
                        is_associated,
                        has_permanent_delegate: false,
                        metadata: TokenMetadata::default(),
                    };
                    accounts.insert(account);
                }
                Ok(_) => unsupported_accounts.push(UnsupportedAccount {
                    address: address.to_string(),
                    err: "Not a token account".to_string(),
                }),
                Err(err) => unsupported_accounts.push(UnsupportedAccount {
                    address: address.to_string(),
                    err: format!("Account parse failure: {err}"),
                }),
            }
        }
    }

    TokenAccounts {
        accounts,
        unsupported_accounts,
        max_len_balance,
        aux_len: if aux_count > 0 {
            format!("  (Aux-{aux_count}*)").chars().count() + 1
        } else {
            0
        },
        explicit_token,
    }
}
