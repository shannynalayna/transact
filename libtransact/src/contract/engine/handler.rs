/*
 * Copyright 2018-2020 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

use crate::contract::{engine::SmartContractEngine, SmartContract};
use crate::handler::{ApplyError, TransactionContext, TransactionHandler};
use crate::protocol::transaction::TransactionPair;

pub struct SmartContractEngineTransactionHandler {
    versions: Vec<String>,
    smart_contract: dyn SmartContract,
}

impl SmartContractEngine for SmartContractEngineTransactionHandler {
    fn apply_smart_contract(
        &self,
        transaction: &TransactionPair,
        context: &mut dyn TransactionContext,
    ) -> Result<(), ApplyError> {
        self.smart_contract.apply(transaction, context)
    }
}

impl TransactionHandler for SmartContractEngineTransactionHandler {
    fn family_name(&self) -> &str {
        "smart_contract_engine_transaction_handler"
    }

    fn family_versions(&self) -> &[String] {
        &self.versions
    }

    fn apply(
        &self,
        transaction: &TransactionPair,
        context: &mut dyn TransactionContext,
    ) -> Result<(), ApplyError> {
        self.apply_smart_contract(transaction, context)
    }
}
