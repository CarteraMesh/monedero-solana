use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

/// Instructions supported by the generic Name Registry program
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum NameRegistryInstruction {
    /// Create an empty name record
    ///
    /// The address of the name record (account #1) is a program-derived address
    /// with the following seeds to ensure uniqueness:
    ///     * SHA256(HASH_PREFIX, `Create::name`)
    ///     * Account class (account #3)
    ///     * Parent name record address (account #4)
    ///
    /// If this is a child record, the parent record's owner must approve by
    /// signing (account #5)
    ///
    /// Accounts expected by this instruction:
    ///   0. `[]` System program
    ///   1. `[writeable, signer]` Funding account (must be a system account)
    ///   2. `[writeable]` Name record to be created (program-derived address)
    ///   3. `[]` Account owner (written into `NameRecordHeader::owner`)
    ///   4. `[signer]` Account class (written into `NameRecordHeader::class`).
    ///      If `Pubkey::default()` then the `signer` bit is not required
    ///   5. `[]` Parent name record (written into
    ///      `NameRecordHeader::parent_name). `Pubkey::default()` is equivalent
    ///      to no existing parent.
    ///   6. `[signer]` Owner of the parent name record. Optional but needed if
    ///      parent name different than default.
    Create {
        /// SHA256 of the (HASH_PREFIX + Name) of the record to create, hashing
        /// is done off-chain
        hashed_name: Vec<u8>,

        /// Number of lamports to fund the name record with
        lamports: u64,

        /// Number of bytes of memory to allocate in addition to the
        /// `NameRecordHeader`
        space: u32,
    },

    /// Update the data in a name record
    ///
    /// Accounts expected by this instruction:
    ///   * If account class is `Pubkey::default()`:
    ///   0. `[writeable]` Name record to be updated
    ///   1. `[signer]` Account owner
    ///
    ///   * If account class is not `Pubkey::default()`:
    ///   0. `[writeable]` Name record to be updated
    ///   1. `[signer]` Account class
    ///
    ///   * If the signer is the parent name account owner
    ///   0. `[writeable]` Name record to be updated
    ///   1. `[signer]` Parent name account owner
    ///   2. `[]` Parent name record
    Update { offset: u32, data: Vec<u8> },

    /// Transfer ownership of a name record
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * If account class is `Pubkey::default()`:
    ///   0. `[writeable]` Name record to be transferred
    ///   1. `[signer]` Account owner
    ///
    ///   * If account class is not `Pubkey::default()`:
    ///   0. `[writeable]` Name record to be transferred
    ///   1. `[signer]` Account owner
    ///   2. `[signer]` Account class
    ///
    ///    * If the signer is the parent name account owner
    ///   0. `[writeable]` Name record to be transferred
    ///   1. `[signer]` Account owner
    ///   2. `[signer]` Account class
    ///   3. `[]` Parent name record
    Transfer { new_owner: Pubkey },

    /// Delete a name record.
    ///
    /// Any lamports remaining in the name record will be transferred to the
    /// refund account (#2)
    ///
    /// Accounts expected by this instruction:
    ///   0. `[writeable]` Name record to be deleted
    ///   1. `[signer]` Account owner
    ///   2. `[writeable]` Refund account
    Delete,

    /// Realloc the data of a name record.
    ///
    /// The space change cannot be more than `MAX_PERMITTED_DATA_LENGTH` greater
    /// than current `space`.
    ///
    /// Accounts expected by this instruction:
    ///   0. `[]` System program
    ///   1. `[writeable, signer]` Payer account (will be refunded if new
    ///      `space` is less than current `space`)
    ///   2. `[writeable]` Name record to be reallocated
    ///   3. `[signer]` Account owner
    Realloc {
        /// New total number of bytes in addition to the `NameRecordHeader`.
        /// There are no checks on the existing data; it will be truncated if
        /// the new space is less than the current space.
        space: u32,
    },
}
solana_program::declare_id!("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");
pub fn transfer(
    new_owner: Pubkey,
    name_account_key: Pubkey,
    name_owner_key: Pubkey,
    name_class_opt: Option<Pubkey>,
) -> Result<Instruction, ProgramError> {
    let instruction_data = NameRegistryInstruction::Transfer { new_owner };
    let data = borsh::to_vec(&instruction_data).unwrap();
    let mut accounts = vec![
        AccountMeta::new(name_account_key, false),
        AccountMeta::new_readonly(name_owner_key, true),
    ];

    if let Some(key) = name_class_opt {
        accounts.push(AccountMeta::new_readonly(key, true));
    }

    Ok(Instruction {
        program_id: ID,
        accounts,
        data,
    })
}
