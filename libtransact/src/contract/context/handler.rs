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

use std::hash::Hash;

use crate::contract::address::Addresser;
use crate::contract::context::key_value::KeyValueTransactionContext;
use crate::handler::{ApplyError, TransactionContext, TransactionHandler};
use crate::protocol::transaction::TransactionPair;

pub trait KeyValueTransactionHandler: Send {
    type Key: Eq + Hash;
    type Addr: Addresser<Self::Key>;

    fn get_family_name(&self) -> &str;

    fn get_family_versions(&self) -> &[String];

    fn get_addresser(&self) -> Self::Addr;

    fn apply<'a>(
        &self,
        transaction: &TransactionPair,
        context: KeyValueTransactionContext<'a, Self::Addr, Self::Key>,
    ) -> Result<(), ApplyError>;
}

impl<T> TransactionHandler for T
where
    T: KeyValueTransactionHandler + Send,
{
    fn family_name(&self) -> &str {
        self.get_family_name()
    }

    fn family_versions(&self) -> &[String] {
        self.get_family_versions()
    }

    fn apply<'a>(
        &self,
        transaction: &TransactionPair,
        context: &'a mut dyn TransactionContext,
    ) -> Result<(), ApplyError> {
        let addr = self.get_addresser();
        let simple_context: KeyValueTransactionContext<'a, T::Addr, T::Key> =
            KeyValueTransactionContext::new(context, addr);
        self.apply(transaction, simple_context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::{Arc, Mutex};

    use crate::contract::address::{
        double_key_hash::DoubleKeyHashAddresser, key_hash::KeyHashAddresser,
        triple_key_hash::TripleKeyHashAddresser,
    };

    use crate::handler::ContextError;

    /// Simple state backed by a HashMap.
    struct TestState {
        state: HashMap<String, Vec<u8>>,
    }

    /// Simple state implementation with basic methods to get, set, and delete state values.
    impl TestState {
        pub fn new() -> Self {
            TestState {
                state: HashMap::new(),
            }
        }

        fn get_entries(
            &self,
            addresses: &[String],
        ) -> Result<Vec<(String, Vec<u8>)>, ContextError> {
            let mut values = Vec::new();
            addresses.iter().for_each(|key| {
                if let Some(value) = self.state.get(key) {
                    values.push((key.to_string(), value.to_vec()))
                }
            });
            Ok(values)
        }

        fn set_entries(&mut self, entries: Vec<(String, Vec<u8>)>) -> Result<(), ContextError> {
            entries.iter().for_each(|(key, value)| {
                match self.state.insert(key.to_string(), value.to_vec()) {
                    _ => (),
                }
            });
            Ok(())
        }

        fn delete_entries(&mut self, addresses: &[String]) -> Result<Vec<String>, ContextError> {
            let mut deleted = Vec::new();
            addresses.iter().for_each(|key| {
                if let Some(_) = self.state.remove(key) {
                    deleted.push(key.to_string());
                }
            });
            Ok(deleted)
        }

        fn add_receipt_data(&self, _data: Vec<u8>) -> Result<(), ContextError> {
            Ok(())
        }

        fn add_event(
            &self,
            _event_type: String,
            _attributes: Vec<(String, String)>,
            _data: Vec<u8>,
        ) -> Result<(), ContextError> {
            Ok(())
        }
    }

    /// TestContext using the simple TestState backend to be used as the internal context to
    /// construct a KeyValueTransactionContext for the tests.
    struct TestContext {
        internal_state: Arc<Mutex<TestState>>,
    }

    impl TestContext {
        pub fn new() -> Self {
            TestContext {
                internal_state: Arc::new(Mutex::new(TestState::new())),
            }
        }
    }

    /// TransactionContext trait implementation for the TestContext.
    impl TransactionContext for TestContext {
        fn get_state_entries(
            &self,
            addresses: &[String],
        ) -> Result<Vec<(String, Vec<u8>)>, ContextError> {
            self.internal_state
                .lock()
                .expect("Test lock was poisoned in get method")
                .get_entries(addresses)
        }

        fn set_state_entries(&self, entries: Vec<(String, Vec<u8>)>) -> Result<(), ContextError> {
            self.internal_state
                .lock()
                .expect("Test lock was poisoned in set method")
                .set_entries(entries)
        }

        fn delete_state_entries(&self, addresses: &[String]) -> Result<Vec<String>, ContextError> {
            self.internal_state
                .lock()
                .expect("Test lock was poisoned in delete method")
                .delete_entries(addresses)
        }

        fn add_receipt_data(&self, data: Vec<u8>) -> Result<(), ContextError> {
            self.internal_state
                .lock()
                .expect("Test lock was poisoned in add_receipt_data method")
                .add_receipt_data(data)
        }

        fn add_event(
            &self,
            event_type: String,
            attributes: Vec<(String, String)>,
            data: Vec<u8>,
        ) -> Result<(), ContextError> {
            self.internal_state
                .lock()
                .expect("Test lock was poisoned in add_event method")
                .add_event(event_type, attributes, data)
        }
    }
}
