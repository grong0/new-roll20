use std::fs;

use serde_json::{from_str, Value};

mod dao;
mod generate_types;
mod update_data;

use dao::races_dao;

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

    let file = fs::read_to_string("../data/raw/races.json").unwrap();
    let races_file: Value = from_str(file.as_str()).unwrap();
    let races_object = races_dao::Races::new(races_file);

    println!("{:#?}", races_object.races);

    println!("{:#?}", races_object.races.get("astral_elf|aag"));
}
