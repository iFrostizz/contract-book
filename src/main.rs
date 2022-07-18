mod cmd;
mod contract;
mod db;

use clap::Parser;
use cmd::parser;
use contract::helper::ContractBook;
use db::config;
use std::env;
use std::fmt;

fn main() {
    let mut db = config::init_db();

    let args = parser::Args::parse();
    let ret_args = parser::parse_args(args).expect("failed to parse args");

    dbg!(&ret_args);

    let chain: String = format!("{}", ret_args.chain);

    db.address.entry(chain).or_default();

    dbg!(&db.address);

    println!("yo");
}
