pub mod handlers;
pub mod instructions;
pub mod state;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

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
