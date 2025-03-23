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

// #[cfg(test)]
// mod test {
//     use crate::state::CounterAccount;

//     use super::*;
//     use borsh::BorshDeserialize;
//     use solana_program_test::*;
//     use solana_sdk::{
//         account::ReadableAccount,
//         hash::Hash,
//         instruction::{AccountMeta, Instruction},
//         signature::{Keypair, Signer},
//         system_instruction, system_program,
//         transaction::{self, Transaction},
//     };

//     async fn initialize_counter() -> (Pubkey, Keypair, Keypair, BanksClient, Hash, u64, u64) {
//         let program_id = Pubkey::new_unique();

//         let program_test = ProgramTest::new(
//             "counter_program",
//             program_id,
//             processor!(process_instruction),
//         );

//         let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

//         let counter_account = Keypair::new();
//         // let authorized_account = Keypair::new();

//         let init_value: u64 = 123;

//         let mut init_data = vec![0];
//         init_data.extend_from_slice(&init_value.to_le_bytes());

//         let init_instruction = Instruction {
//             program_id,
//             accounts: vec![
//                 solana_sdk::instruction::AccountMeta::new(counter_account.pubkey(), true),
//                 solana_sdk::instruction::AccountMeta::new(payer.pubkey(), true),
//                 solana_sdk::instruction::AccountMeta::new_readonly(
//                     solana_program::system_program::id(),
//                     false,
//                 ),
//             ],
//             data: init_data,
//         };

//         let mut transaction =
//             Transaction::new_with_payer(&[init_instruction], Some(&payer.pubkey()));

//         transaction.sign(&vec![&payer, &counter_account], recent_blockhash);

//         banks_client.process_transaction(transaction).await.unwrap();

//         let counter_acc_data = banks_client
//             .get_account(counter_account.pubkey())
//             .await
//             .expect("Failed to fetch account")
//             .expect("Account does not exist");

//         let counter_state = CounterAccount::try_from_slice(&counter_acc_data.data())
//             .expect("failed to deserialize");

//         assert_eq!(
//             counter_state.counter, 123,
//             "Counter state mismatch: expected 123, got {}",
//             counter_state.counter
//         );
//         println!(
//             "Success! Counter was initialized to {}",
//             counter_state.counter
//         );

//         return (
//             program_id,
//             counter_account,
//             payer,
//             banks_client,
//             recent_blockhash,
//             counter_state.counter,
//             counter_state.update_count,
//         );
//     }

//     #[tokio::test]
//     async fn test_initialize_counter() {
//         let counter = initialize_counter().await;

//         assert_eq!(counter.5, 123);
//         println!("Success! Counter was initialized to {}", counter.5);
//     }

//     #[tokio::test]
//     async fn test_counter_increment() {
//         //data from initialization
//         let (
//             program_id,
//             counter_account,
//             payer_account,
//             mut banks_client,
//             recent_blockhash,
//             mut initial_counter_value,
//             mut initial_update_value,
//         ) = initialize_counter().await;

//         let increment_data = vec![1u8];
//         let decrement_data = vec![2u8];

//         let increment_instruction = Instruction {
//             program_id,
//             accounts: vec![solana_sdk::instruction::AccountMeta::new(
//                 counter_account.pubkey(),
//                 false,
//             )],
//             data: increment_data,
//         };

//         let mut increment_transaction =
//             Transaction::new_with_payer(&[increment_instruction], Some(&payer_account.pubkey()));

//         increment_transaction.sign(&vec![&payer_account], recent_blockhash);

//         banks_client
//             .process_transaction(increment_transaction)
//             .await
//             .unwrap();

//         let counter_account_data = banks_client
//             .get_account(counter_account.pubkey())
//             .await
//             .expect("Failed to fetch account")
//             .expect("Account does not exist");

//         let counter_state = CounterAccount::try_from_slice(&counter_account_data.data())
//             .expect("failed to deserialize");

//         let mut res = counter_state.counter == (initial_counter_value + 1);

//         assert!(
//             res,
//             "Account not incremented, expected {}, found {}",
//             (initial_counter_value + 1),
//             counter_state.counter
//         );

//         let mut update_res = counter_state.update_count == (initial_update_value + 1);

//         assert!(
//             update_res,
//             "Update Counter not updated, expected {}, found {}",
//             (initial_counter_value + 1),
//             counter_state.update_count
//         );

//         println!(
//             "Success! Counter was incremented to {}
//             Current Update count: {}",
//             counter_state.counter, counter_state.update_count
//         );

//         //Decrement
//         initial_counter_value = counter_state.counter;
//         initial_update_value = counter_state.update_count;

//         let decrement_instruction = Instruction {
//             program_id,
//             accounts: vec![solana_sdk::instruction::AccountMeta::new(
//                 counter_account.pubkey(),
//                 false,
//             )],
//             data: decrement_data,
//         };

//         let mut decrement_transaction =
//             Transaction::new_with_payer(&[decrement_instruction], Some(&payer_account.pubkey()));

//         decrement_transaction.sign(&vec![&payer_account], recent_blockhash);

//         banks_client
//             .process_transaction(decrement_transaction)
//             .await
//             .unwrap();

//         let counter_account_data = banks_client
//             .get_account(counter_account.pubkey())
//             .await
//             .expect("Failed to fetch account")
//             .expect("Account does not exist");

//         let counter_state = CounterAccount::try_from_slice(&counter_account_data.data())
//             .expect("failed to deserialize");

//         res = counter_state.counter == (initial_counter_value - 1);
//         update_res = counter_state.update_count == (initial_update_value + 1);

//         assert!(
//             res,
//             "Account not decremented, expected {}, found {}",
//             (initial_counter_value - 1),
//             counter_state.counter
//         );
//         assert!(
//             update_res,
//             "Update Counter not updated, expected {}, found {}",
//             (initial_counter_value + 1),
//             counter_state.update_count
//         );

//         println!(
//             "Success! Counter was decremented to {}
//             Current Update count: {}",
//             counter_state.counter, counter_state.update_count
//         );
//     }

//     #[tokio::test]
//     async fn test_set_counter_value() {
//         //data from initialization
//         let (
//             program_id,
//             counter_account,
//             payer_account,
//             mut banks_client,
//             recent_blockhash,
//             initial_counter_value,
//             initial_update_value,
//         ) = initialize_counter().await;

//         let value: u64 = 50;
//         let mut in_data = vec![3u8];
//         in_data.extend_from_slice(&value.to_le_bytes());

//         let set_instructions = Instruction {
//             program_id,
//             accounts: vec![
//                 solana_sdk::instruction::AccountMeta::new(payer_account.pubkey(), true),
//                 solana_sdk::instruction::AccountMeta::new(counter_account.pubkey(), true),
//             ],
//             data: in_data,
//         };

//         let mut set_transaction =
//             Transaction::new_with_payer(&[set_instructions], Some(&payer_account.pubkey()));
//         set_transaction.sign(&vec![&payer_account, &counter_account], recent_blockhash);

//         banks_client
//             .process_transaction(set_transaction)
//             .await
//             .unwrap();

//         let counter_account_data = banks_client
//             .get_account(counter_account.pubkey())
//             .await
//             .expect("Failed to fetch account")
//             .expect("Account does not exist");

//         let counter_state = CounterAccount::try_from_slice(&counter_account_data.data())
//             .expect("failed to deserialize");

//         let res = counter_state.counter == value;

//         assert!(
//             res,
//             "Counter not updated, expected {}, found {}",
//             value, counter_state.counter
//         );

//         println!(
//             "Success! Counter was set to {}
//             Current Update count: {}",
//             counter_state.counter, counter_state.update_count
//         );
//     }
// }
