use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CounterAccount {
    pub counter: u64,
    pub update_count: u64,
    pub owner: [u8; 32],
}
