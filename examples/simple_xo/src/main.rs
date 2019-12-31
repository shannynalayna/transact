// Copyright 2019 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod handler;

pub fn main() {}
// fn create_executor(context_manager: &ContextManager) -> Arc<Mutex<Option<Executor>>> {
//     Arc::new(Mutex::new(Some(Executor::new(vec![Box::new(
//         StaticExecutionAdapter::new_adapter(
//             vec![Box::new(SawtoothToTransactHandlerAdapter::new(
//                 XoTransactionHandler::new(),
//             ))],
//             context_manager.clone(),
//         )
//         .expect("Unable to create static execution adapter"),
//     )]))))
// }
//
// fn start_executor(executor: &Arc<Mutex<Option<Executor>>>) {
//     executor
//         .lock()
//         .expect("Should not have poisoned the lock")
//         .as_mut()
//         .expect("Should not be None")
//         .start()
//         .expect("Start should not have failed");
// }
//
// fn create_batch(signer: &dyn Signer, game_address: &str, payload: &str) -> BatchPair {
//     let txn_pair = TransactionBuilder::new()
//         .with_batcher_public_key(signer.public_key().to_vec())
//         .with_family_name("xo".to_string())
//         .with_family_version("2.0".to_string())
//         .with_inputs(vec![hex::decode(&game_address).unwrap()])
//         .with_nonce(b"test_nonce".to_vec())
//         .with_outputs(vec![hex::decode(&game_address).unwrap()])
//         .with_payload_hash_method(HashMethod::SHA512)
//         .with_payload(payload.as_bytes().to_vec())
//         .build_pair(signer)
//         .expect("The TransactionBuilder was not given the correct items");
//
//     BatchBuilder::new()
//         .with_transactions(vec![txn_pair.take().0])
//         .build_pair(signer)
//         .expect("Unable to build batch a pair")
// }
//
// fn run_schedule(executor: &Arc<Mutex<Option<Executor>>>, scheduler: &mut dyn Scheduler) {
//     let task_iterator = scheduler
//         .take_task_iterator()
//         .expect("Failed to take task iterator");
//     executor
//         .lock()
//         .expect("Should not have poisoned the lock")
//         .as_ref()
//         .expect("Should not be None")
//         .execute(
//             task_iterator,
//             scheduler.new_notifier().expect("Failed to get notifier"),
//         )
//         .expect("Failed to execute schedule");
// }
//
// fn get_receipt(batch_result: BatchExecutionResult) -> TransactionReceipt {
//     assert_eq!(1, batch_result.results.len());
//
//     let mut batch_result = batch_result;
//
//     let txn_result = batch_result
//         .results
//         .pop()
//         .expect("Length 1, but no first element");
//     match txn_result {
//         TransactionExecutionResult::Valid(receipt) => receipt,
//         TransactionExecutionResult::Invalid(invalid_result) => {
//             panic!("Transaction failed: {:?}", invalid_result)
//         }
//     }
// }
