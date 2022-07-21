use std::collections::HashMap;

use ethers::abi::Abi;
use ethers::types::Address;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CoreContract {
    pub abi: Abi,
    pub address: HashMap<u64, Address>,
}

// name => CoreContract
pub type ContractBook = HashMap<String, CoreContract>;
