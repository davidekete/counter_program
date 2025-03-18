mod handlers;
pub mod instructions;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
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

#[cfg(test)]
mod test {
    use super::*;
    use solana_program_test::*;
    use solana_sdk::{
        account::ReadableAccount,
        instruction::{AccountMeta, Instruction},
        signature::{Keypair, Signer},
        system_instruction, system_program,
        transaction::{self, Transaction},
    };

    #[tokio::test]
    async fn test_initialize_counter() {
        let program_id = Pubkey::new_unique();

        let mut program_test =
            ProgramTest::new("counter", program_id, processor!(process_instruction));

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let counter_account = Keypair::new();

        // let rent = banks_client.get_rent().await.unwrap();
        // let account_space = std::mem::size_of::<CounterAccount>();
        // let required_lamports = rent.minimum_balance(account_space);

        // let create_counter_account_ix = system_instruction::create_account(
        //     &payer.pubkey(),
        //     &counter_account.pubkey(),
        //     required_lamports,
        //     account_space as u64,
        //     &program_id,
        // );

        let init_value: u64 = 123;

        println!("Testing counter initialization...");

        let mut init_data = vec![0];
        init_data.extend_from_slice(&init_value.to_le_bytes());

        let init_instruction = Instruction {
            program_id,
            accounts: vec![
                solana_sdk::instruction::AccountMeta::new(counter_account.pubkey(), true),
                solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
                solana_sdk::instruction::AccountMeta::new_readonly(
                    solana_program::system_program::id(),
                    false,
                ),
            ],
            data: init_data,
        };

        let mut transaction =
            Transaction::new_with_payer(&[init_instruction], Some(&payer.pubkey()));

        transaction.sign(&vec![&payer, &counter_account], recent_blockhash);

        banks_client.send_transaction(transaction).await.unwrap();

        let counter_acc_data = banks_client
            .get_account(counter_account.pubkey())
            .await
            .expect("Failed to fetch account")
            .expect("Account does not exist");

        let counter_state = CounterAccount::try_from_slice(&counter_acc_data.data())
            .expect("failed to deserialize");

        assert_eq!(counter_state.counter, 123);
        println!(
            "Success! Counter was initialized to {}",
            counter_state.counter
        );
    }
}
