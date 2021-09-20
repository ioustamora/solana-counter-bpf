pub struct Counter {
    pub counter: u32,
    pub value: i64,
};

pub struct Settings {
    owner: [u8, 32],
    inc_step: u32,
    dec_step: u32,
};