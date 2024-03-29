use clap::Parser;
use crate::db::store::wipe_db;

#[derive(Parser, Debug)]
pub struct Utils {
    #[clap(short, long, help = "reset db")]
    pub reset: bool,
}

pub fn utils_args(args: Utils) -> eyre::Result<()> {
    if args.reset {
        wipe_db()?;
    }

    Ok(())
}
