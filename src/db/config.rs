use eyre::{ContextCompat, WrapErr};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::path::PathBuf;

use crate::contract::helper::ContractBook;

pub fn init_db() -> ContractBook {
    let book_path = get_book_path().unwrap();

    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&book_path)
        .expect("coudln't open db file");

    let buf_len = fs::metadata(&book_path).unwrap().len();

    if buf_len == 0 {
        println!("db is empty");
        return HashMap::new();
    }

    let reader = BufReader::new(&file);

    serde_json::from_reader(reader).unwrap()
}

pub fn get_db_path() -> eyre::Result<PathBuf> {
    let path = dirs_next::data_dir()
        .wrap_err("Failed to find data directory")?
        .join("cbook");
    fs::create_dir_all(&path).wrap_err("Failed to create module directory")?;

    Ok(path)
}

pub fn get_book_path() -> eyre::Result<PathBuf> {
    let path = get_db_path().unwrap();
    let path = path.join("db.json");

    Ok(path)
}
