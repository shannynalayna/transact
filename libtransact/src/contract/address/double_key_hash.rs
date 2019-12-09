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

pub struct DoubleKeyHashAddresser {
    prefix: String,
    first_hash_length: usize,
}

impl DoubleKeyHashAddresser {
    pub fn new(prefix: String, first_hash_length: Option<usize>) -> DoubleKeyHashAddresser {
        DoubleKeyHashAddresser {
            prefix: prefix.clone(),
            first_hash_length: first_hash_length.unwrap_or((ADDRESS_LENGTH - prefix.len()) / 2),
        }
    }
}

impl Addresser<(String, String)> for DoubleKeyHashAddresser {
    fn compute(&self, key: &(String, String)) -> Result<String, AddresserError> {
        let hash_length = ADDRESS_LENGTH - self.prefix.len();
        let second_hash_length = hash_length - self.first_hash_length;
        if (self.prefix.len() + self.first_hash_length + second_hash_length) != ADDRESS_LENGTH {
            return Err(AddresserError::DoubleKeyHashAddresserError(
                "Invalid hash length".to_string(),
            ));
        }
        let first_hash = &hash(self.first_hash_length, &key.0);
        let second_hash = &hash(second_hash_length, &key.1);

        Ok(String::from(&self.prefix) + first_hash + second_hash)
    }

    fn normalize(&self, key: &(String, String)) -> String {
        key.0.to_string() + "_" + &key.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // check that the DoubleKeyHashAddresser creates a valid radix address with the correct prefix
    // and valid default length, with a key represented as a tuple with two natural keys
    fn test_double_key_default_length() {
        let addresser = DoubleKeyHashAddresser::new("prefix".to_string(), None);
        let key1 = "a";
        let key1_hash = hash(32, key1);
        let key2 = "b";
        let key2_hash = hash(32, key2);

        let addr = addresser
            .compute(&(key1.to_string(), key2.to_string()))
            .unwrap();
        assert_eq!(addr[..6], "prefix".to_string());
        assert_eq!(addr.len(), 70);

        assert_eq!(addr[6..38], key1_hash[..32]);
        assert_eq!(addr[38..], key2_hash[..32]);

        let normalized = addresser.normalize(&(key1.to_string(), key2.to_string()));
        assert_eq!(normalized, "a_b".to_string());
    }

    #[test]
    // check that the DoubleKeyHashAddresser creates a valid radix address with the correct prefix
    // and valid first hash length of 16, with a key represented as a tuple with two natural keys
    fn test_double_key_custom_length() {
        let addresser = DoubleKeyHashAddresser::new("prefix".to_string(), Some(16));
        let key1 = "a";
        let key1_hash = hash(16, key1);
        let key2 = "b";
        let key2_hash = hash(48, key2);

        let addr = addresser
            .compute(&(key1.to_string(), key2.to_string()))
            .unwrap();
        assert_eq!(addr[..6], "prefix".to_string());
        assert_eq!(addr.len(), 70);

        assert_eq!(addr[6..22], key1_hash[..16]);
        assert_eq!(addr[22..], key2_hash[..48]);

        let normalized = addresser.normalize(&(key1.to_string(), key2.to_string()));
        assert_eq!(normalized, "a_b".to_string());
    }
}
