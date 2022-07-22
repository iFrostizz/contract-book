use crate::{
    contract::helper::{ContractBook, CoreContract},
    parser::RetArgs,
};

use serde_json;
use std::collections::{hash_map::Entry, HashMap};
use std::fs;

use ethers::abi::Abi;

pub fn store_from_args(db: &mut ContractBook, args: RetArgs) -> &mut ContractBook {
    dbg!(&args.force);

    let core_contract = match db.entry(args.name) {
        Entry::Vacant(entry) => {
            let address = args.chain.map_or_else(
                || HashMap::new(),
                |chain| HashMap::from_iter([(chain, args.address.unwrap())]),
            );

            entry.insert(CoreContract {
                abi: args.abi,
                address: address,
            });

            return db;
        }
        Entry::Occupied(entry) => {
            dbg!("Already have dis name");
            entry.into_mut()
        }
    };

    // dbg!(&core_contract);

    match &core_contract.abi {
        Some(abi) if args.force => core_contract.abi = args.abi,
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
                } else {
                    panic!("address already present, force to overwrite");
                }
            }
        },
        _ => (),
    }

    db
}

pub fn write_to_db(db: &mut ContractBook, file: fs::File) {
    dbg!("gonna write", &db);
    serde_json::to_writer_pretty(file, &db).expect("could not write to db");
    dbg!("wrote to db");
}
