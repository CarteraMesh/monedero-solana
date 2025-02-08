use {super::Instructor, solana_sdk::instruction::Instruction};

impl Instructor {
    pub fn memo(&self, message: &str) -> Instruction {
        Instruction {
            program_id: spl_memo::id(),
            accounts: vec![],
            data: message.as_bytes().to_vec(),
        }
    }
}

#[cfg(test)]
mod test {
    use {crate::Instructor, std::str::FromStr, wasm_client_solana::SolanaRpcClient};

    #[test]
    fn memo() -> anyhow::Result<()> {
        let payer =
            solana_pubkey::Pubkey::from_str("215r9xfTFVYcE9g3fAUGowauM84egyUvFCbSo3LKNaep")?;
        let rpc = SolanaRpcClient::new(wasm_client_solana::DEVNET);
        let instructor = Instructor::new(&payer, &rpc);
        let i = instructor.memo("blah");
        assert_eq!(i.program_id, spl_memo::id());
        Ok(())
    }
}
