use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string},
};

pub mod actions_dao;
pub mod backgrounds_dao;
pub mod character_dao;
pub mod classes_dao;
pub mod common;
pub mod conditionsdiseases_dao;
pub mod feats_dao;
pub mod items_dao;
pub mod languages_dao;
pub mod races_dao;
pub mod skills_dao;
pub mod spells_dao;

use actions_dao::Action;
use backgrounds_dao::Background;
use character_dao::Character;
use classes_dao::{Class, ClassFeature, Subclass, SubclassFeature};
use common::serde_as_array;
use conditionsdiseases_dao::{Condition, Disease, Status};
use feats_dao::Feat;
use items_dao::Item;
use languages_dao::Language;
use races_dao::Race;
use serde_json::{from_str, to_value, Map, Value};
use skills_dao::Skill;
use spells_dao::Spell;

fn get_actions(path: &str) -> HashMap<String, Action> {
    let file = read_to_string(path);
    if file.is_err() {
        return HashMap::new();
    }
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
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
    if file.is_err() {
        return HashMap::new();
    }
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
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

fn get_characters(dir_path: &str) -> HashMap<String, Character> {
    let files = read_dir(dir_path);
    if files.is_err() {
        return HashMap::new();
    }

    let map = HashMap::new();
    for entry in files.unwrap() {
        let file = read_to_string(entry.unwrap().path());
        if file.is_err() {
            continue;
        }
        let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
        // let new_struct = Character::new(serde_file);
        // map.insert(new_struct.key, new_struct);
    }

    return map;
}

fn get_classes_and_subclasses(paths: Vec<&str>) -> (HashMap<String, Class>, HashMap<String, Subclass>, HashMap<String, ClassFeature>, HashMap<String, SubclassFeature>) {
    let mut class_value_list: Vec<Value> = vec![];
    let mut subclass_value_list: Vec<Value> = vec![];
    let mut class_feature_value_list: Vec<Value> = vec![];
    let mut subclass_feature_value_list: Vec<Value> = vec![];
    for path in paths {
        let file = read_to_string(path);
        if file.is_err() {
            continue;
        }
        let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
        let single_class_value_list: Vec<Value> = serde_as_array(serde_file.get("class"));
        let single_subclass_value_list: Vec<Value> = serde_as_array(serde_file.get("subclass"));
        let single_class_feature_value_list: Vec<Value> = serde_as_array(serde_file.get("classFeature"));
        let single_subclass_feature_value_list: Vec<Value> = serde_as_array(serde_file.get("subclassFeature"));

        class_value_list.extend(single_class_value_list);
        subclass_value_list.extend(single_subclass_value_list);
        class_feature_value_list.extend(single_class_feature_value_list);
        subclass_feature_value_list.extend(single_subclass_feature_value_list);
    }
    println!("total num of classes: {}", class_value_list.len());
    println!("total num of subclasses: {}", subclass_value_list.len());
    println!("total num of class features: {}", class_feature_value_list.len());
    println!("total num of subclass features: {}", subclass_feature_value_list.len());

    let mut num_of_na = 0;
    let mut class_map: HashMap<String, Class> = HashMap::new();
    for value in class_value_list {
        let new_struct = Class::new(value);
        if !new_struct.key.contains("n/a") {
            class_map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of classes with no name: {}", num_of_na);

    let mut num_of_na = 0;
    let mut subclass_map: HashMap<String, Subclass> = HashMap::new();
    for value in subclass_value_list {
        let new_struct = Subclass::new(value);
        if !new_struct.key.contains("n/a") {
            subclass_map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of subclasses with no name: {}", num_of_na);

    let mut num_of_na = 0;
    let mut class_feature_map: HashMap<String, ClassFeature> = HashMap::new();
    for value in class_feature_value_list {
        let new_struct = ClassFeature::new(value);
        if !new_struct.key.contains("n/a") {
            class_feature_map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of class features with no name: {}", num_of_na);

    let mut num_of_na = 0;
    let mut subclass_feature_map: HashMap<String, SubclassFeature> = HashMap::new();
    for value in subclass_feature_value_list {
        let new_struct = SubclassFeature::new(value);
        if !new_struct.key.contains("n/a") {
            subclass_feature_map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of subclass features with no name: {}", num_of_na);

    return (class_map, subclass_map, class_feature_map, subclass_feature_map);
}

fn get_conditions_and_diseases(path: &str) -> (HashMap<String, Condition>, HashMap<String, Disease>, HashMap<String, Status>) {
    let file = read_to_string(path);
    if file.is_err() {
        return (HashMap::new(), HashMap::new(), HashMap::new());
    }
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
    let condition_list: Vec<Value> = serde_as_array(serde_file.get("condition"));
    println!("num of conditions: {}", condition_list.len());
    let disease_list: Vec<Value> = serde_as_array(serde_file.get("disease"));
    println!("num of diseases: {}", disease_list.len());
    let status_list: Vec<Value> = serde_as_array(serde_file.get("status"));
    println!("num of statuses: {}", status_list.len());

    let mut num_of_na = 0;
    let mut condition_map: HashMap<String, Condition> = HashMap::new();
    for value in condition_list {
        let new_struct = Condition::new(value);
        if !new_struct.key.contains("n/a") {
            condition_map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of conditions with no name: {}", num_of_na);

    let mut num_of_na = 0;
    let mut disease_map: HashMap<String, Disease> = HashMap::new();
    for value in disease_list {
        let new_struct = Disease::new(value);
        if !new_struct.key.contains("n/a") {
            disease_map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of diseases with no name: {}", num_of_na);

    let mut num_of_na = 0;
    let mut status_map: HashMap<String, Status> = HashMap::new();
    for value in status_list {
        let new_struct = Status::new(value);
        if !new_struct.key.contains("n/a") {
            status_map.insert(new_struct.key.as_str().to_string(), new_struct);
        } else {
            num_of_na += 1;
        }
    }
    println!("number of statuses with no name: {}", num_of_na);

    return (condition_map, disease_map, status_map);
}

fn get_feats(path: &str) -> HashMap<String, Feat> {
    let file = read_to_string(path);
    if file.is_err() {
        return HashMap::new();
    }
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
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
    if file.is_err() {
        return HashMap::new();
    }
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
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
    if file.is_err() {
        return HashMap::new();
    }
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
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
    if file.is_err() {
        return HashMap::new();
    }
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
    let value_list: Vec<Value> = serde_as_array(serde_file.get("race"));
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
    if file.is_err() {
        return HashMap::new();
    }
    let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
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
        if file.is_err() {
            continue;
        }
        let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
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
    pub characters: HashMap<String, Character>,
    pub classes: HashMap<String, Class>,
    pub class_features: HashMap<String, ClassFeature>,
    pub conditions: HashMap<String, Condition>,
    pub diseases: HashMap<String, Disease>,
    pub feats: HashMap<String, Feat>,
    pub items: HashMap<String, Item>,
    pub languages: HashMap<String, Language>,
    pub races: HashMap<String, Race>,
    pub skills: HashMap<String, Skill>,
    pub spells: HashMap<String, Spell>,
    pub statuses: HashMap<String, Status>,
    pub subclasses: HashMap<String, Subclass>,
    pub subclass_features: HashMap<String, SubclassFeature>,
}

impl DAO {
    pub fn new() -> DAO {
        let class_path_list: Vec<&str> = vec![
            "data/raw/class/class-artificer.json",
            "data/raw/class/class-barbarian.json",
            "data/raw/class/class-bard.json",
            "data/raw/class/class-cleric.json",
            "data/raw/class/class-druid.json",
            "data/raw/class/class-fighter.json",
            "data/raw/class/class-monk.json",
            "data/raw/class/class-mystic.json",
            "data/raw/class/class-paladin.json",
            "data/raw/class/class-ranger.json",
            "data/raw/class/class-rogue.json",
            "data/raw/class/class-sidekick.json",
            "data/raw/class/class-sorcerer.json",
            "data/raw/class/class-warlock.json",
            "data/raw/class/class-wizard.json",
        ];

        let (classes, subclasses, class_features, subclass_features) = get_classes_and_subclasses(class_path_list);

        let spell_path_list: Vec<&str> = vec![
            "data/raw/spells/spells-aag.json",
            "data/raw/spells/spells-ai.json",
            "data/raw/spells/spells-aitfr-avt.json",
            "data/raw/spells/spells-bmt.json",
            "data/raw/spells/spells-dodk.json",
            "data/raw/spells/spells-egw.json",
            "data/raw/spells/spells-ftd.json",
            "data/raw/spells/spells-ggr.json",
            "data/raw/spells/spells-ghloe.json",
            "data/raw/spells/spells-idrotf.json",
            "data/raw/spells/spells-llk.json",
            "data/raw/spells/spells-phb.json",
            "data/raw/spells/spells-sato.json",
            "data/raw/spells/spells-scc.json",
            "data/raw/spells/spells-tce.json",
            "data/raw/spells/spells-tdcsr.json",
            "data/raw/spells/spells-xge.json",
        ];

        let (conditions, diseases, statuses) = get_conditions_and_diseases("data/raw/conditionsdiseases.json");

        return DAO {
            actions: get_actions("data/raw/actions.json"),
            backgrounds: get_backgrounds("data/raw/backgrounds.jsons"),
            characters: get_characters("data/characters"),
            classes,
            class_features,
            conditions,
            diseases,
            feats: get_feats("data/raw/feats.json"),
            items: get_items("data/raw/items.json"),
            languages: get_languages("data/raw/languages.json"),
            races: get_races("data/raw/races.json"),
            skills: get_skills("data/raw/skills.json"),
            spells: get_spells(spell_path_list),
            statuses,
            subclasses,
            subclass_features,
        };
    }
}
