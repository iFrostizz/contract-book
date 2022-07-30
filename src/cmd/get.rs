use clap::Parser;
use ethers::types::Address;
use eyre::{eyre, Error, Result};
use std::collections::HashMap;

use crate::{
    cmd::parser::{parse_chain, print_db},
    contract::helper::{ContractBook, CoreContract},
};

#[derive(Parser, Debug)]
pub struct Get {
    #[clap(short, long, help = "contract name")]
    pub name: Option<String>,

    #[clap(short, long, help = "chain name or id", requires = "name")]
    pub chain: Option<String>,

    #[clap(short, long, help = "needs only address(es)", requires = "chain")]
    pub address: bool,

    #[clap(long, help = "need only abi")]
    pub abi: bool,
}

pub fn get_args(db: &mut ContractBook, args: Get) -> eyre::Result<()> {
    // let mut db_name = &db[&args.name];
    if let Some(name) = args.name {
        let db_name = db.get_mut(&name).ok_or_else(|| eyre!("Name not set"))?;

        match args.chain {
            Some(chain) => {
                // can display address as well
                let chain = parse_chain(chain)?;
                let db_chain = db_name
                    .address
                    .get_mut(&chain)
                    .ok_or_else(|| eyre!("Chain not set for {name}"))?;

                if (args.abi && args.address) || (!args.abi && !args.address) {
                    let asked_addr = db_chain.clone();
                    let mut local_addr = HashMap::new();
                    local_addr.insert(chain, asked_addr);
                    db_name.address = local_addr;

                    print_db(db_name);
                } else if args.address {
                    print_db(&db_chain);
                } else if args.abi {
                    print_db(&db_name.abi);
                }
            }
            None => {
                if args.abi {
                    print_db(&db_name.abi);
                } else {
                    print_db(db_name);
                }
            }
        }
    } else {
        // Dump all db
        print_db(db);
    }

    Ok(())
}
