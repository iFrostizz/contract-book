mod db;
mod contract;

use db::config::init_db;
use contract::helper::ContractBook;

fn main() {
    let db = init_db().unwrap();
}
