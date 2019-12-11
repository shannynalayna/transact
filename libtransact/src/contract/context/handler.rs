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
