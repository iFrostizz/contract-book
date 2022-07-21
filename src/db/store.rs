use crate::{
    contract::helper::{ContractBook, CoreContract},
    parser::RetArgs,
};

use serde_json;
use std::collections::{hash_map::Entry, HashMap};
use std::fs;

use ethers::abi::Abi;

pub fn store_from_args(db: &mut ContractBook, args: RetArgs) -> &mut ContractBook {
    let core_contract = match db.entry(args.name) {
        Entry::Vacant(entry) => {
            let address = args.chain.map_or_else(
                || HashMap::new(),
                |chain| HashMap::from_iter([(chain, args.address.unwrap())]),
            );

            entry.insert(CoreContract {
                abi: Abi::default(),
                address: address,
            });

            return db;
        }
        Entry::Occupied(entry) => entry.into_mut(),
    };

    match args.abi {
        Some(abi) if args.force => core_contract.abi = abi,
        Some(abi) => panic!("ABI already present, force to overwrite"), // not forced
        _ => (),
    }

    match args.address {
        Some(address) => match core_contract.address.entry(args.chain.unwrap()) {
            Entry::Vacant(entry) => {
                entry.insert(args.address.unwrap());
            }
            Entry::Occupied(mut entry) => {
                if args.force {
                    entry.insert(args.address.unwrap());
                }
            }
        },
        _ => (),
    }

    db
}

pub fn write_to_db(db: &mut ContractBook, file: fs::File) {
    serde_json::to_writer_pretty(file, &db).expect("could not write to db");
    dbg!("wrote to db");
}
