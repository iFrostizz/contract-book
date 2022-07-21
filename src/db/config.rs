use eyre::{ContextCompat, Result, WrapErr};
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};
use std::path::PathBuf;

use crate::contract::helper::ContractBook;

pub fn init_db() -> (ContractBook, fs::File) {
    let book_path = get_book_path().unwrap();

    dbg!(&book_path);

    let mut file = fs::File::create(book_path).expect("coudln't open db file");

    let reader = BufReader::new(&file);
    let buf_len = reader.buffer().len();

    if buf_len == 0 {
        dbg!("empty db");

        // file.write(b"{}").unwrap(); // initialize json file for parsing

        return (HashMap::new(), file);
    }

    let db = serde_json::from_reader(reader).unwrap();

    (db, file)
}

fn get_db_path() -> Result<PathBuf> {
    let path = dirs_next::data_dir()
        .wrap_err("Failed to find data directory")?
        .join("cbook");
    fs::create_dir_all(&path).wrap_err("Failed to create module directory")?;

    Ok(path)
}

fn get_book_path() -> Result<PathBuf> {
    let path = get_db_path().unwrap();
    let path = path.join("db.json");

    Ok(path)
}
