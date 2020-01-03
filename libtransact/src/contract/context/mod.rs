/*
 * Copyright 2019 Cargill Incorporated
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

pub mod error;
#[cfg(feature = "contract-context-key-value")]
pub mod key_value;

use std::collections::HashMap;

use crate::contract::address::Addresser;
use crate::contract::context::error::ContractContextError;
use crate::handler::TransactionContext;

pub trait ContractContext<'a, A, K>
where
    A: Addresser<K>,
{
    type State_Value;

    fn make_context(context: &'a mut dyn TransactionContext, addresser: A) -> Self;

    fn set_state_entries(
        &self,
        entries: HashMap<&K, HashMap<String, Self::State_Value>>,
    ) -> Result<(), ContractContextError>;

    fn get_state_entries(
        &self,
        keys: Vec<&K>,
    ) -> Result<HashMap<String, HashMap<String, Self::State_Value>>, ContractContextError>;

    fn delete_state_entries(&self, keys: Vec<K>) -> Result<Vec<String>, ContractContextError>;
}
