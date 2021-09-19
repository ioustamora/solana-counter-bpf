#![cfg(feature = "test-bpf")]

use {
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
};

#[test]
fn test_validator_transaction() {
    let program_id = Pubkey::new_unique();
    Ok()
}
