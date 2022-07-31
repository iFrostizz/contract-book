mod cmd;
mod contract;
mod db;

use clap::Parser;

use cmd::parser;
use db::config;

fn main() {
    let mut db = config::init_db();

    let args = parser::Args::parse();

    parser::process_args(&mut db, args)
        .unwrap_or_else(|err| println!("\x1b[31mcbook err: {}\x1b[0m", err));
}
