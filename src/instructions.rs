pub enum CounterInstruction {
    //update settings
    //accounts:
    // 0. '[signer, writable]' owner 
    // 1. '[writable]' settings account, PDA
    // 2. '[]' Rent sysvar
    // 3. '[]' System program
    Set,
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