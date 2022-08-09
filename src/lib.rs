pub mod cmd;
pub mod contract;
pub mod db;

use crate::{cmd::parser::parse_chain, db::config::init_db};
use ethers::types::Address;
use eyre::eyre;

#[allow(dead_code)]
pub struct AddressArgs {
    pub name: String,
    pub chain: String,
}

pub fn get_address(args: AddressArgs) -> eyre::Result<Address> {
    let db = init_db();

    Ok(*db
        .get(&args.name)
        .ok_or_else(|| eyre!("Could not get name"))?
        .address
        .get(&parse_chain(args.chain)?)
        .ok_or_else(|| eyre!("Could not get chain"))?)
}
