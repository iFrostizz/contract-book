use clap::Parser;
use eyre::Result;

use crate::db::store::wipe_db;

#[derive(Parser, Debug)]
pub struct Utils {
    #[clap(short, long, help = "reset db")]
    pub reset: bool,
}

pub fn utils_args(args: Utils) -> Result<()> {
    if args.reset {
        wipe_db()?;
    }

    Ok(())
}
