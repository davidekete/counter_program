use crate::state::CounterAccount;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    value: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let counter_account = next_account_info(accounts_iter)?;
    if counter_account.owner != program_id && !counter_account.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let payer_account = next_account_info(accounts_iter)?;
    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let system_program = next_account_info(accounts_iter)?;

    let account_space = std::mem::size_of::<CounterAccount>();
    let rent = Rent::get()?;

    let required_lamports = rent.minimum_balance(account_space);
    msg!(
        "Allocating {} bytes, required lamports = {}",
        account_space,
        required_lamports
    );

    invoke(
        &system_instruction::create_account(
            payer_account.key,
            counter_account.key,
            required_lamports,
            account_space as u64,
            program_id,
        ),
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone(),
        ],
    )?;

    let counter_data = CounterAccount {
        counter: value,
        update_count: 0,
        owner: payer_account.key.to_bytes(),
    };

    let mut account_data = &mut counter_account.data.borrow_mut()[..];

    // Serialize the CounterAccount struct into the account's data
    counter_data.serialize(&mut account_data)?;

    msg!("Counter initialized with value: {}", value);

    Ok(())
}

pub fn process_increment(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let counter_account = next_account_info(accounts_iter)?;

    // Verify account ownership
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Mutable borrow the account data
    let mut data = counter_account.data.borrow_mut();

    let mut counter_data: CounterAccount = CounterAccount::try_from_slice(&data)?;

    counter_data.counter = counter_data
        .counter
        .checked_add(1)
        .ok_or(ProgramError::InvalidAccountData)?;

    counter_data.update_count = counter_data
        .update_count
        .checked_add(1)
        .ok_or(ProgramError::InvalidAccountData)?;

    counter_data.serialize(&mut &mut data[..])?;

    msg!("Counter incremented to: {}", counter_data.counter);

    Ok(())
}

pub fn process_decrement(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let counter_account = next_account_info(accounts_iter)?;

    // Verify account ownership
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Mutable borrow the account data
    let mut data = counter_account.data.borrow_mut();

    let mut counter_data: CounterAccount = CounterAccount::try_from_slice(&data)?;

    counter_data.counter = counter_data
        .counter
        .checked_sub(1)
        .ok_or(ProgramError::InvalidAccountData)?;

    counter_data.update_count = counter_data
        .update_count
        .checked_add(1)
        .ok_or(ProgramError::InvalidAccountData)?;

    counter_data.serialize(&mut &mut data[..])?;

    msg!("Counter incremented to: {}", counter_data.counter);

    Ok(())
}

pub fn process_set_value(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    value: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // 1) The user (attempting to set the counter)
    let signer_info = next_account_info(accounts_iter)?;

    if !signer_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let counter_account = next_account_info(accounts_iter)?;

    // Verify account ownership
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Mutable borrow the account data
    let mut data = counter_account.data.borrow_mut();

    let mut counter_data: CounterAccount = CounterAccount::try_from_slice(&data)?;

    if signer_info.key.to_bytes() != counter_data.owner {
        return Err(ProgramError::IllegalOwner);
    }

    println!("This is the value {}", value);

    counter_data.counter = value;

    counter_data.update_count = counter_data
        .update_count
        .checked_add(1)
        .ok_or(ProgramError::InvalidAccountData)?;

    counter_data.serialize(&mut &mut data[..])?;

    Ok(())
}
