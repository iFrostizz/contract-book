use crate::cmd::parser::Args;

use std::collections::HashMap;
use std::str::FromStr;

use ethers::abi::Abi;
use ethers::types::{Address, Chain};

use serde::{Serialize, Deserialize};

use eyre::Result;

pub type AddressBook = HashMap<String, HashMap<String, Address>>;
pub type AbiBook = HashMap<String, Abi>;

#[derive(Serialize, Deserialize)]
pub struct ContractBook {
    pub address: AddressBook,
    pub abi: AbiBook,
}

/*pub fn build_entry(args: Args) -> ContractBook {
    // let address = Address::from_str(args.address)?;

}*/
