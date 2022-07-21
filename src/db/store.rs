use crate::{
    contract::helper::{ContractBook, CoreContract},
    parser::RetArgs,
};

use serde_json;
use std::collections::hash_map;
use std::fs;

pub fn store_from_args(mut db: ContractBook, args: RetArgs) -> ContractBook {
    // db.entry(args.name).or_default().entry();

    let db_name = db.entry(args.name).or_default();

    if args.abi.is_some() {
        db_name.abi = args.abi.unwrap(); // TODO: use "get" to handle force
    }

    if args.address.is_some() {
        /*let db_address = db_name.address;

        match db_address.entry(args.chain.unwrap()) {
            hash_map::VacantEntry(entry) => {
                entry.insert(args.address.unwrap());
            }
            _ => {
                if args.force {
                    entry.insert(args.address.unwrap());
                }
            }
        }

        db_address
            .entry(args.chain.unwrap())
            .or_default()
            .entry(args.address.unwrap())
            .or_default();*/
    }

    db
}

pub fn write_to_db(db: ContractBook, file: fs::File) {
    serde_json::to_writer_pretty(file, &db).expect("could not write to db");
    dbg!("wrote to db");
}
