use std::{collections::HashMap, fs::read_to_string};

pub mod actions_dao;
pub mod backgrounds_dao;
pub mod common;
pub mod feats_dao;
pub mod items_dao;
pub mod languages_dao;
pub mod races_dao;
pub mod skills_dao;
pub mod spells_dao;

use actions_dao::Action;
use backgrounds_dao::Background;
use common::{serde_as_array, serde_as_object};
use feats_dao::Feat;
use items_dao::Item;
use languages_dao::Language;
use races_dao::Race;
use serde::de::value;
use serde_json::{from_str, to_value, Map, Value};
use skills_dao::Skill;
use spells_dao::Spell;

fn get_races(path: &str) -> HashMap<String, Race> {
	let file = read_to_string(path).unwrap();
	let races_file: Value = from_str(file.as_str()).unwrap();
	let value_list: Vec<Value> = serde_as_array(races_file.get("races"));
	println!("num of races: {}", value_list.len());

	let mut num_of_na = 0;
	let mut races_map: HashMap<String, Race> = HashMap::new();
	for value in value_list {
		let new_race = Race::new(Some(&value));
		if !new_race.key.contains("n/a") {
			races_map.insert(new_race.key.as_str().to_string(), new_race);
		} else {
			num_of_na += 1;
		}
	}
	println!("number of races with no name: {}", num_of_na);

	return races_map;
}

#[derive(Debug)]
pub struct DAO {
    pub actions: HashMap<String, Action>,
    pub backgrounds: HashMap<String, Background>,
    pub feats: HashMap<String, Feat>,
    pub items: HashMap<String, Item>,
    pub languages: HashMap<String, Language>,
    pub races: HashMap<String, Race>,
    pub skills: HashMap<String, Skill>,
    pub spells: HashMap<String, Spell>,
}

impl DAO {
	pub fn new() -> DAO {
		return DAO {
			actions: get_actions(),
			backgrounds: get_backgrounds(),
			feats: get_feats(),
			items: get_items(),
			languages: get_languages(),
			races: get_races("../data/raw/races.json"),
			skills: get_skills(),
			spells: get_spells()
		}
	}
}
