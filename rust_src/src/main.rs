use std::fs;

use serde_json::{from_str, Value};

mod dao;
mod generate_types;
mod update_data;

use dao::{races_dao, DAO};

fn main() {
    // generate_types::build();
    // generate_types::testing_build();
    // let file = fs::read_dir("data");
    // if file.is_ok() {
    // 	println!("{:?}", file);
    // } else {
    // 	println!("no such file or directory")
    // }

    // update_data::update_data();
	let DAO = DAO::new();

    println!("{:#?}", DAO);

    println!("{:#?}", DAO.races.get("astral_elf|aag"));
}
