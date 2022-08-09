use clap::Parser;

use cbook::cmd::parser::{process_args, Args};
use cbook::db::config::init_db;

fn main() {
    let mut db = init_db();

    let args = Args::parse();

    process_args(&mut db, args).unwrap_or_else(|err| println!("\x1b[31mcbook err: {}\x1b[0m", err));
}
