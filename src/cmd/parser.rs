    use std::str::FromStr;

use clap::Parser;
use ethers::abi::{Abi, Address};
use ethers::types::{Chain, ParseChainError};

use eyre::{Result, Error};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, help = "contract address")]
    pub address: Option<Address>,
    #[clap(long, help = "contract abi")]
    pub abi: Option<String>,
    #[clap(long, help = "chain name or id")]
    pub chain: String,
    #[clap(short, help = "reset the database", default_value_t = false)]
    pub reset: bool 
}

// TODO: store abi as name no matter the chain
#[derive(Debug)]
pub struct RetArgs {
    pub chain: Chain
}

pub fn parse_args(args: Args) -> Result<RetArgs, Error> {
    let chain: String = args.chain;

    let chain: Chain = Chain::from_str(&chain)
        .or_else(|_| Ok::<_, eyre::Report>(Chain::try_from(chain.parse::<u64>()?)?))?;

    let ret = RetArgs {
        chain
    };

    Ok(ret)
}
