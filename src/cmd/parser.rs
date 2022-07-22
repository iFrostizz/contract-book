use std::str::FromStr;

use clap::Parser;
use ethers::abi;
use ethers::abi::{Abi, AbiEncode, Address};
use ethers::types::Chain;

use ethereum_abi::Abi as BetterAbi;

use eyre::{Error, Result};

#[derive(Parser, Debug)]
pub struct Args {
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

pub fn parse_args(args: Args) -> Result<RetArgs, Error> {
    let chain = if args.chain.is_some() {
        let chain: String = args.chain.unwrap();

        let chain: Chain = Chain::from_str(&chain)
            .or_else(|_| Ok::<_, eyre::Report>(Chain::try_from(chain.parse::<u64>()?)?))?;

        Some(chain.into())
    } else {
        None
    };

    if args.address.is_none() {
        if args.abi.is_none() {
            panic!("Provide address or abi")
        }
    } else {
        if chain.is_none() {
            panic!("Should provide a chain with address")
        }
    }

    let abi: Option<Abi> = match args.abi.is_some() {
        true => Some(serde_json::from_str(&args.abi.unwrap()).unwrap()),
        _ => None,
    };

    let ret = RetArgs {
        name: args.name,
        chain,
        address: args.address,
        abi,
        force: args.force,
    };

    Ok(ret)
}
