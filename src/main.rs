mod cmd;
mod contract;
mod db;

use clap::Parser;
use cmd::parser;
use db::{config, store};

fn main() {
    let mut db = config::init_db();

    let args = parser::Args::parse();
    let ret_args = parser::parse_args(args).expect("failed to parse args");

    let db = store::store_from_args(&mut db, ret_args);

    store::write_to_db(db);
}
