use std::fs;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::path::Path;
use std::collections::HashMap;
use std::sync::Arc;

use crate::contract::helper::ContractBook;

use eyre::Result;

const DB_PATH: &str = "/var/lib/contract-book/";

pub fn init_db() -> ContractBook {
    match fs::create_dir_all(DB_PATH) {
        Err(err) => {
            if err.kind() == ErrorKind::PermissionDenied {
                panic!("cbook require root privilege to create the db");
            }
        },
        _ => {}
    }; 

    let book_path: &str = &(DB_PATH.to_owned() + "db.json");

    let mut file = fs::File::create(book_path).expect("coudln't open db file");

    let reader = BufReader::new(&file);
    let buf_len = reader.buffer().len();

    serde_json::from_reader(reader).unwrap_or_else(|err| {
        // dbg!(&err);

        if buf_len == 0 {
            dbg!("empty db");

            file.write(b"{}").unwrap();

            return ContractBook {
                address: HashMap::new(),
                abi: HashMap::new()
            };
        }

        panic!("{}", &err);
    })
}
