use std::collections::HashMap;
use ethers::abi::Abi;
use ethers::types::{Address, Chain};

pub type AddressBook = HashMap<Chain, HashMap<String, Address>>;
pub type AbiBook = HashMap<String, Abi>;
pub struct ContractBook {
    address: AddressBook,
    abi: AbiBook
}