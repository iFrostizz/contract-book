use kv::{Config, Error, Store};
// use eyre::Result;

pub fn init_db() -> Result<Store, Error> {
    let mut cfg = Config::new("~/var/lib/contract-book/db");
    Store::new(cfg)
}