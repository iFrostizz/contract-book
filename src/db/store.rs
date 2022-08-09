use crate::{contract::helper::ContractBook, db::config::get_book_path};

use eyre::Result;
use serde_json::to_writer_pretty;

use std::fs::{write, File, OpenOptions};

fn get_file() -> eyre::Result<File> {
    let book_path = get_book_path()?;

    Ok(OpenOptions::new().write(true).open(&book_path)?)
}

pub fn write_to_db(db: &mut ContractBook) -> eyre::Result<()> {
    let file = get_file()?;
    to_writer_pretty(file, &db)?;

    Ok(())
}

pub fn wipe_db() -> eyre::Result<()> {
    write(get_book_path()?, "")?;

    Ok(())
}
