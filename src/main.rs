mod cmd;
mod contract;
mod db;

use clap::Parser;
use cmd::parser;
// use contract::helper::ContractBook;
use db::{config, store};

fn main() {
    let (mut db, file) = config::init_db();

    let args = parser::Args::parse();
    let ret_args = parser::parse_args(args).expect("failed to parse args");

    dbg!(&ret_args);

    let db = store::store_from_args(db, ret_args);

    store::write_to_db(db, file);

    /*let chain: String = format!("{}", ret_args.chain);

    db.entry(chain).or_default();

    dbg!(&db);

    println!("yo");*/
}
