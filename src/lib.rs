mod handlers;
pub mod instructions;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

// solana_program::declare_id!("Counter111111111111111111111111111111111");

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CounterAccount {
    pub counter: u64,
    pub update_count: u64,
    pub owner: [u8; 32],
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instructions::CounterInstruction::unpack(instruction_data)?;

    match instruction {
        instructions::CounterInstruction::Initialize(value) => {
            handlers::process_initialize(program_id, accounts, value)
        }
        instructions::CounterInstruction::Increment => {
            handlers::process_increment(program_id, accounts)
        }
        instructions::CounterInstruction::Decrement => {
            handlers::process_decrement(program_id, accounts)
        }
        instructions::CounterInstruction::SetValue(value) => {
            handlers::process_set_value(program_id, accounts, value)
        }
    }
}
