mod add;
mod initialize;
mod set;
mod sub;

use add::*;
use initialize::*;

use example_program_api::prelude::*;
use set::process_set;
use steel::*;
use sub::process_sub;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&example_program_api::ID, program_id, data)?;

    match ix {
        ExampleProgramInstruction::Initialize => process_initialize(accounts, data)?,
        ExampleProgramInstruction::Add => process_add(accounts, data)?,
        ExampleProgramInstruction::Sub => process_sub(accounts, data)?,
        ExampleProgramInstruction::Set => process_set(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
