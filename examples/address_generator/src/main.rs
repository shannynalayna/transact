//
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

use clap::{App, Arg};

use transact::contract::address::{
    double_key_hash::DoubleKeyHashAddresser, key_hash::KeyHashAddresser,
    triple_key_hash::TripleKeyHashAddresser, Addresser,
};

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("prefix")
                .long("prefix")
                .takes_value(true)
                .required(true)
                .help("the prefix of the radix address"),
        )
        .arg(
            Arg::with_name("keys")
                .required(true)
                .takes_value(true)
                .max_values(3)
                .help("the natural key used to compute the radix address"),
        )
        .get_matches();
    let prefix = matches
        .value_of("prefix")
        .expect("Unable to get `prefix` arg")
        .to_string();
    let keys: Vec<_> = matches
        .values_of("keys")
        .expect("Unable to get `keys` arg")
        .collect();
    match keys.len() {
        1 => {
            let addresser = KeyHashAddresser::new(prefix);
            let addr = addresser
                .compute(&keys[0].to_string())
                .expect("Unable to compute hash");
            println!("{:?}", addr);
        }
        2 => {
            let addresser = DoubleKeyHashAddresser::new(prefix, None);
            let addr = addresser
                .compute(&(keys[0].to_string(), keys[1].to_string()))
                .expect("Unable to compute hash");
            println!("{:?}", addr);
        }
        3 => {
            let addresser = TripleKeyHashAddresser::new(prefix, None, None);
            let addr = addresser
                .compute(&(
                    keys[0].to_string(),
                    keys[1].to_string(),
                    keys[2].to_string(),
                ))
                .expect("Unable to compute hash");
            println!("{:?}", addr);
        }
        _ => {
            println!("Unable to compute radix address");
        }
    }
}
