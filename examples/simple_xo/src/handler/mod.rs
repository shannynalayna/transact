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

use std::marker::PhantomData;

use transact::contract::{
    address::{key_hash::KeyHashAddresser, Addresser},
    context::key_value::KeyValueTransactionContext,
    handler::SmartContract,
};
use transact::handler::{ApplyError, TransactionContext};
use transact::protocol::transaction::TransactionPair;

struct XoSmartContract<'a, T: 'a>
where
    T: Send + Sync,
{
    family_name: String,
    family_versions: Vec<String>,
    addresser: KeyHashAddresser,
    _lifetime: PhantomData<&'a T>,
}

impl<'a, T: 'a> XoSmartContract<'a, T>
where
    T: Send + Sync,
{
    pub fn new(addresser: KeyHashAddresser) -> Self {
        XoSmartContract {
            family_name: "xo2".to_string(),
            family_versions: vec!["0.1".to_string()],
            addresser,
            _lifetime: PhantomData,
        }
    }
}

impl<'b, T> SmartContract for XoSmartContract<'b, T>
where
    T: Send + Sync,
{
    type Key = String;
    type Addr = KeyHashAddresser;
    type Context = KeyValueTransactionContext<'b, KeyHashAddresser, String>;

    fn get_family_name(&self) -> &str {
        &self.family_name
    }

    fn get_family_versions(&self) -> &[String] {
        &self.family_versions
    }

    fn get_addresser(&self) -> Self::Addr {
        self.addresser.clone()
    }

    fn make_context<'a>(
        &self,
        addresser: KeyHashAddresser,
        context: &'a mut dyn TransactionContext,
    ) -> &mut Self::Context {
        &mut KeyValueTransactionContext::new(context, addresser)
    }

    fn apply(
        &self,
        transaction: &TransactionPair,
        context: &mut Self::Context,
    ) -> Result<(), ApplyError> {
        unimplemented!()
    }
}
