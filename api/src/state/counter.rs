use steel::*;

use super::ExampleProgramAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64,
    pub update_count: u64,
    pub authority: Pubkey,
}

account!(ExampleProgramAccount, Counter);
