use borsh::{
    BorshSerialize,
    BorshDeserialize,
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

use crate::{
    id,
    state::{Counter, Settings},
};

pub enum CounterInstruction {
    //update settings
    //accounts:
    // 0. '[signer, writable]' owner 
    // 1. '[writable]' settings account, PDA
    // 2. '[]' Rent sysvar
    // 3. '[]' System program
    Set {
        admin: [u8, 32],
        inc_step: u32,
        dec_step: u32,
    },
    // increments counter
    // accounts:
    // 0. '[signer]' owner
    // 1. '[writable]' counter account, PDA
    // 2. '[]' settings account, PDA
    Inc,
    // decrements counter
    //accounts:
    // 0. '[signer]' owner
    // 1. '[writable]' counter account, PDA
    // 2. '[writable]' settings account, PDA
    Dec,
}