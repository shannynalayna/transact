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

use crate::contract::address::{hash, Addresser, AddresserError, ADDRESS_LENGTH};

pub struct KeyHashAddresser {
    prefix: String,
}

impl KeyHashAddresser {
    pub fn new(prefix: String) -> KeyHashAddresser {
        KeyHashAddresser { prefix }
    }
}

impl Addresser<String> for KeyHashAddresser {
    fn compute(&self, key: &String) -> Result<String, AddresserError> {
        let hash_length = ADDRESS_LENGTH - self.prefix.len();

        Ok(String::from(&self.prefix) + &hash(hash_length, key))
    }

    fn normalize(&self, key: &String) -> String {
        key.to_string()
    }
}

#[cfg(feature = "contract-address-test")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // check that the KeyHashAddresser creates a valid radix address with the correct prefix
    // and valid length
    fn test_key_hash_addresser() {
        let addresser = KeyHashAddresser::new("prefix".to_string());
        let addr = addresser.compute(&"a".to_string()).unwrap();
        assert_eq!(addr[..6], "prefix".to_string());
        assert_eq!(addr.len(), 70);

        let key_hash = hash(64, "a");
        assert_eq!(addr[6..70], key_hash[..64]);

        let normalized = addresser.normalize(&"b".to_string());
        assert_eq!(normalized, "b".to_string());
    }
}
