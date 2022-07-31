use std::str::FromStr;

use clap::{Parser, Subcommand};
use ethers::types::{Address, Chain};
use serde::Serialize;

use crate::{
    cmd::{get, store, utils},
    contract::helper::ContractBook,
};

use eyre::{Error, Result};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Store(store::Store),
    Get(get::Get),
    Utils(utils::Utils),
}

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

pub fn process_args(db: &mut ContractBook, args: Args) -> Result<(), Error> {
    match args.command {
        Commands::Store(store) => Ok(store::store_args(db, store)?),
        Commands::Get(get) => Ok(get::get_args(db, get)?),
        Commands::Utils(utils) => Ok(utils::utils_args(utils)?),
    }
}

pub fn parse_chain(chain: String) -> eyre::Result<u64> {
    let chain: Chain = Chain::from_str(&chain)
        .or_else(|_| Ok::<_, eyre::Report>(Chain::try_from(chain.parse::<u64>()?)?))?;

    let chain: u64 = chain.into();

    Ok(chain)
}

pub fn print_db<T: Serialize>(db: T) {
    println!("{}", serde_json::to_string_pretty(&db).unwrap());
}

pub fn print_addr(addr: Address) {
    println!("{:?}", addr);
}
