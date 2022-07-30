use clap::Parser;
use ethers::{
    abi::Abi,
    types::{Address, Chain},
};
use eyre::{Error, Result};
use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

use crate::{
    contract::helper::{ContractBook, CoreContract},
    db::store::write_to_db,
};

#[derive(Parser, Debug)]
pub struct Store {
    #[clap(long, help = "contract name")]
    pub name: String,

    #[clap(long, help = "contract address")]
    pub address: Option<Address>,

    #[clap(long, help = "contract abi")]
    pub abi: Option<String>,

    #[clap(long, help = "chain name or id")]
    pub chain: Option<String>,

    #[clap(long, help = "overwrite")]
    pub force: bool,
}

#[derive(Debug)]
pub struct RetArgs {
    pub name: String,
    pub chain: Option<u64>,
    pub address: Option<Address>,
    pub abi: Option<Abi>,
    pub force: bool,
}

pub fn store_args(db: &mut ContractBook, args: Store) -> Result<(), Error> {
    let ret_args = parse_args(db, args)?;
    let db = store_from_args(db, ret_args)?;
    write_to_db(db);

    Ok(())
}

pub fn parse_args(db: &mut ContractBook, args: Store) -> Result<RetArgs, Error> {
    let chain: Option<u64> = if args.chain.is_some() {
        let chain: String = args.chain.unwrap();

        let chain: Chain = Chain::from_str(&chain)
            .or_else(|_| Ok::<_, eyre::Report>(Chain::try_from(chain.parse::<u64>()?)?))?;

        Some(chain.into())
    } else {
        None
    };

    if args.address.is_none() {
        if args.abi.is_none() {
            eyre::bail!("Please provide address or abi")
        }
    } else {
        if chain.is_none() {
            eyre::bail!("You should provide a chain with address")
        }
    }

    let abi: Option<Abi> = match args.abi.is_some() {
        true => Some(serde_json::from_str(&args.abi.unwrap()).unwrap()),
        _ => None,
    }; // TODO: add hint to put single quotes around ABI

    let ret = RetArgs {
        name: args.name,
        chain,
        address: args.address,
        abi,
        force: args.force,
    };

    Ok(ret)
}

/// Returns the whole db
pub fn store_from_args(db: &mut ContractBook, args: RetArgs) -> Result<&mut ContractBook, Error> {
    let core_contract = match db.entry(args.name) {
        Entry::Vacant(entry) => {
            let address = args.chain.map_or_else(
                || HashMap::new(),
                |chain| HashMap::from_iter([(chain, args.address.unwrap())]),
            );

            entry.insert(CoreContract {
                abi: args.abi,
                address: address,
            });

            return Ok(db);
        }
        Entry::Occupied(entry) => entry.into_mut(),
    };

    match &core_contract.abi {
        Some(_abi) if args.force => core_contract.abi = args.abi,
        Some(_abi) => eyre::bail!("ABI already present, force to overwrite"), // not forced
        _ => (),
    }

    match args.address {
        Some(_address) => match core_contract.address.entry(args.chain.unwrap()) {
            Entry::Vacant(entry) => {
                entry.insert(args.address.unwrap());
            }
            Entry::Occupied(mut entry) => {
                if args.force {
                    entry.insert(args.address.unwrap());
                } else {
                    eyre::bail!("address already present, force to overwrite");
                }
            }
        },
        _ => (),
    }

    Ok(db)
}
