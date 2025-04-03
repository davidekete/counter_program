use example_program_api::prelude::*;
use steel::*;

pub fn process_set(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    let value = u64::from_le_bytes(Add::try_from_bytes(data)?.amount);

    let [signer_info, counter_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    let counter = counter_info
        .as_account_mut::<Counter>(&example_program_api::ID)?
        .assert_mut(|c| c.value < 100)?;

    if !(signer_info.key == &counter.authority) {
        return Err(ProgramError::from(ExampleProgramError::WrongSigner));
    }

    // Update state
    counter.value = value;

    Ok(())
}
