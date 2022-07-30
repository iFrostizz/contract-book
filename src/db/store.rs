use crate::{
    contract::helper::{ContractBook, CoreContract},
    db::config,
};

use serde_json;
use std::collections::{hash_map::Entry, HashMap};
use std::fs;

pub fn write_to_db(db: &mut ContractBook) {
    let book_path = config::get_book_path().unwrap();

    let file = fs::OpenOptions::new()
        .write(true)
        .open(&book_path)
        .expect("coudln't open db file");

    serde_json::to_writer_pretty(file, &db).expect("could not write to db");
}
