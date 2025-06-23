mod dao;
mod version_checking;

use dao::DAO;
use version_checking::{is_newer_version, update_data};

const UPDATE_DATA: bool = false;

fn main() {
    if UPDATE_DATA && is_newer_version() {
        let result = update_data();
        if result.is_err() {
            println!("{:?}", result.err());
        };
    }

    let dao = DAO::new();
}
