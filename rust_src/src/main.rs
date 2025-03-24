use std::fs;

use serde_json::{Value, from_str};

mod races;
mod update_data;
mod generate_types;

fn main() {
	generate_types::build();
	// generate_types::testing_build();
	// let file = fs::read_dir("data");
	// if file.is_ok() {
	// 	println!("{:?}", file);
	// } else {
	// 	println!("no such file or directory")
	// }

	// update_data::update_data();



    // let file = fs::read_to_string("data/raw/races.json").unwrap();
    // let races_file: Value = from_str(file.as_str()).unwrap();
    // let races_object = races::Races::new(races_file);

    // for race in races_object.races {
    //     println!("{:?}", race);
    // }
}
