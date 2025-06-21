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
use common::serde_as_array;
use feats_dao::Feat;
use items_dao::Item;
use languages_dao::Language;
use races_dao::Race;
use serde_json::{from_str, Value};
use skills_dao::Skill;
use spells_dao::Spell;

fn get_actions(path: &str) -> HashMap<String, Action> {
    let file = read_to_string(path);
	if !file.is_ok() {
		return HashMap::new();
	}
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap();
    let value_list: Vec<Value> = serde_as_array(serde_file.get("action"));
    println!("num of actions: {}", value_list.len());

    let mut num_of_na = 0;
    let mut map: HashMap<String, Action> = HashMap::new();
    for value in value_list {
        let new_struct = Action::new(value);
        if !new_struct.key.contains("n/a") {
            map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of actions with no name: {}", num_of_na);

    return map;
}

fn get_backgrounds(path: &str) -> HashMap<String, Background> {
    let file = read_to_string(path);
	if !file.is_ok() {
		return HashMap::new();
	}
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap();
    let value_list: Vec<Value> = serde_as_array(serde_file.get("background"));
    println!("num of backgrounds: {}", value_list.len());

    let mut num_of_na = 0;
    let mut map: HashMap<String, Background> = HashMap::new();
    for value in value_list {
        let new_struct = Background::new(value);
        if !new_struct.key.contains("n/a") {
            map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of backgrounds with no name: {}", num_of_na);

    return map;
}

fn get_feats(path: &str) -> HashMap<String, Feat> {
    let file = read_to_string(path);
	if !file.is_ok() {
		return HashMap::new();
	}
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap();
    let value_list: Vec<Value> = serde_as_array(serde_file.get("feat"));
    println!("num of feats: {}", value_list.len());

    let mut num_of_na = 0;
    let mut map: HashMap<String, Feat> = HashMap::new();
    for value in value_list {
        let new_struct = Feat::new(value);
        if !new_struct.key.contains("n/a") {
            map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of feats with no name: {}", num_of_na);

    return map;
}

fn get_items(path: &str) -> HashMap<String, Item> {
    let file = read_to_string(path);
	if !file.is_ok() {
		return HashMap::new();
	}
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap();
    let value_list: Vec<Value> = serde_as_array(serde_file.get("item"));
    println!("num of items: {}", value_list.len());

    let mut num_of_na = 0;
    let mut map: HashMap<String, Item> = HashMap::new();
    for value in value_list {
        let new_struct = Item::new(value);
        if !new_struct.key.contains("n/a") {
            map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of items with no name: {}", num_of_na);

    return map;
}

fn get_languages(path: &str) -> HashMap<String, Language> {
    let file = read_to_string(path);
	if !file.is_ok() {
		return HashMap::new();
	}
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap();
    let value_list: Vec<Value> = serde_as_array(serde_file.get("language"));
    println!("num of languages: {}", value_list.len());

    let mut num_of_na = 0;
    let mut map: HashMap<String, Language> = HashMap::new();
    for value in value_list {
        let new_struct = Language::new(value);
        if !new_struct.key.contains("n/a") {
            map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of languages with no name: {}", num_of_na);

    return map;
}

fn get_races(path: &str) -> HashMap<String, Race> {
    let file = read_to_string(path);
	if !file.is_ok() {
		return HashMap::new();
	}
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap();
    let value_list: Vec<Value> = serde_as_array(serde_file.get("races"));
    println!("num of races: {}", value_list.len());

    let mut num_of_na = 0;
    let mut map: HashMap<String, Race> = HashMap::new();
    for value in value_list {
        let new_struct = Race::new(value);
        if !new_struct.key.contains("n/a") {
            map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of races with no name: {}", num_of_na);

    return map;
}

fn get_skills(path: &str) -> HashMap<String, Skill> {
    let file = read_to_string(path);
	if !file.is_ok() {
		return HashMap::new();
	}
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap();
    let value_list: Vec<Value> = serde_as_array(serde_file.get("skill"));
    println!("num of skills: {}", value_list.len());

    let mut num_of_na = 0;
    let mut map: HashMap<String, Skill> = HashMap::new();
    for value in value_list {
        let new_struct = Skill::new(value);
        if !new_struct.key.contains("n/a") {
            map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of skills with no name: {}", num_of_na);

    return map;
}

fn get_spells(paths: Vec<&str>) -> HashMap<String, Spell> {
    let mut value_list: Vec<Value> = vec![];
    for path in paths {
        let file = read_to_string(path);
		if !file.is_ok() {
			continue;
		}
        let serde_file: Value = from_str(file.unwrap().as_str()).unwrap();
        let single_value_list: Vec<Value> = serde_as_array(serde_file.get("spell"));
        println!("num of spells in '{}': {}", path, single_value_list.len());

        value_list.extend(single_value_list);
    }
    println!("total num of spells: {}", value_list.len());

    let mut num_of_na = 0;
    let mut map: HashMap<String, Spell> = HashMap::new();
    for value in value_list {
        let new_struct = Spell::new(value);
        if !new_struct.key.contains("n/a") {
            map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of spells with no name: {}", num_of_na);

    return map;
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
        let spell_path_list: Vec<&str> = vec![
            "../data/raw/spells/spells-aag.json",
            "../data/raw/spells/spells-ai.json",
            "../data/raw/spells/spells-aitfr-avt.json",
            "../data/raw/spells/spells-bmt.json",
            "../data/raw/spells/spells-dodk.json",
            "../data/raw/spells/spells-egw.json",
            "../data/raw/spells/spells-ftd.json",
            "../data/raw/spells/spells-ggr.json",
            "../data/raw/spells/spells-ghloe.json",
            "../data/raw/spells/spells-idrotf.json",
            "../data/raw/spells/spells-llk.json",
            "../data/raw/spells/spells-phb.json",
            "../data/raw/spells/spells-sato.json",
            "../data/raw/spells/spells-scc.json",
            "../data/raw/spells/spells-tce.json",
            "../data/raw/spells/spells-tdcsr.json",
            "../data/raw/spells/spells-xge.json",
        ];

        return DAO {
            actions: get_actions("../data/raw/actions.json"),
            backgrounds: get_backgrounds("../data/raw/backgrounds.jsons"),
            feats: get_feats("../data/raw/feats.json"),
            items: get_items("../data/raw/items.json"),
            languages: get_languages("../data/raw/languages.json"),
            races: get_races("../data/raw/races.json"),
            skills: get_skills("../data/raw/skills.json"),
            spells: get_spells(spell_path_list),
        };
    }
}
