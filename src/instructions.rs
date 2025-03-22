use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

pub enum CounterInstruction {
    Initialize(u64),
    Increment,
    Decrement,
    SetValue(u64),
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //first byte is the enum variant, the rest are the instructions
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match variant {
            0 => {
                // Initialize
                // Expect 8 bytes for the initial value (u64)
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let initial_value = u64::from_le_bytes(
                    rest.try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                println!("This is the initial value: {}", initial_value);
                Ok(Self::Initialize(initial_value))
            }
            1 => Ok(Self::Increment),
            2 => Ok(Self::Decrement),
            3 => {
                if rest.len() < 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let value = u64::from_le_bytes(
                    rest.try_into()
                        .map_err(|_| ProgramError::InvalidInstructionData)?,
                );
                Ok(Self::SetValue(value))
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        }
    }
}
