use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{id, COUNTER_SEED, SETTINGS_SEED};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub counter: u32,
    pub value: i64,
}

impl Counter {
    pub fn get_counter_pubkey(user: &Pubkey) -> Pubkey {
        Pubkey::create_with_seed(user, COUNTER_SEED, &id()).unwrap()
    }

    pub fn is_ok_counter_pubkey(user: &Pubkey, counter: &Pubkey) -> bool {
        counter.to_bytes() == Self::get_counter_pubkey(user).to_bytes()
    }
}

pub struct Settings {
    owner: [u8, 32],
    inc_step: u32,
    dec_step: u32,
}

impl Settings {
    pub fn get_settings_pubkey_with_bump() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SETTINGS_SEED.as_bytes()], &id())
    }

    pub fn get_settings_pubkey() -> Pubkey {
        let (pubkey, _) = Self::get_settings_pubkey_with_bump();
        pubkey
    }
}