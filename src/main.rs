use std::fs;

use serde_json::{Value, from_str};

mod races;

fn main() {
    let file = fs::read_to_string("data/raw/races.json").unwrap();
    let races_file: Value = from_str(file.as_str()).unwrap();
    let races_object = races::Races::new(races_file);

    for race in races_object.races {
        println!("{:?}", race);
    }
}
