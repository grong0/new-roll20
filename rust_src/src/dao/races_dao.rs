use std::collections::HashMap;

use serde_json::{to_value, Value};

use crate::dao::misc_classes::{Source, Speed};

use super::misc_classes::{Ability, AdditionalSpells, Age, ArmorProficiencies, HeightAndWeight, LanguageProficiencies, Resist, SkillProficiencies, ToolProficiencies, WeaponProficiencies};

#[derive(Debug)]
pub struct ModifyRaceCopy {
    mode: String,
    name: String,
    entries: Vec<String>,
}

impl ModifyRaceCopy {
    pub fn new(entry: &Value) -> ModifyRaceCopy {
        let mut entries: Vec<String> = vec![];
        for entry in entry.get("items").unwrap().get("entries").unwrap_or(&to_value::<Vec<String>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter() {
            entries.push(entry.as_str().unwrap_or("N/A").to_string());
        }
        return ModifyRaceCopy {
            mode: entry.get("mode").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            name: entry.get("items").unwrap().get("name").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            entries,
        };
    }
}

#[derive(Debug)]
pub struct RaceCopy {
    name: String,
    source: String,
    modifying: Vec<ModifyRaceCopy>,
    preserving: Vec<String>,
}

impl RaceCopy {
    pub fn new(copy: Option<&Value>) -> RaceCopy {
        if copy.is_none() {
            return RaceCopy {
                name: "N/A".to_string(),
                source: "N/A".to_string(),
                modifying: vec![],
                preserving: vec![],
            };
        }
        let mut modifying: Vec<ModifyRaceCopy> = vec![];
        for modify in copy.unwrap().get("_mod").unwrap().get("entries").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter() {
            modifying.push(ModifyRaceCopy::new(modify))
        }
        let mut preserving: Vec<String> = vec![];
        for modify in copy.unwrap().get("_preserve").unwrap().as_object().unwrap().keys().into_iter() {
            preserving.push(modify.to_owned());
        }
        return RaceCopy {
            name: copy.unwrap().get("name").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            source: copy.unwrap().get("source").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            modifying,
            preserving,
        };
    }
}

#[derive(Debug)]
pub struct Race {
    pub name: String,
    pub key: String,
    pub source: Source,
    pub srd: bool,
    pub basic_rules: bool,
    pub other_sources: Vec<Source>,
    pub reprinted_as: Vec<String>,
    // pub copy: RaceCopy,
    pub lineage: String,
    pub creature_types: Vec<String>,
    pub creature_type_tags: Vec<String>,
    pub size: Vec<String>,
    pub speed: Speed,
    pub ability: Ability,
    pub height_and_weight: HeightAndWeight,
    pub age: Age,
    pub darkvision: u64,
    pub trait_tags: Vec<String>,
    pub skill_proficiencies: SkillProficiencies,
    pub language_proficiencies: LanguageProficiencies,
    pub tool_proficiencies: ToolProficiencies,
    pub weapon_proficiencies: WeaponProficiencies,
    pub armor_proficiencies: ArmorProficiencies,
    pub resist: Resist,
    pub additional_spells: AdditionalSpells,
    // pub immune: Vec<String>,
    // pub condition_immune: Vec<String>,
    // pub entries: HashMap<&str, Vec<String>>,
    // pub has_fluff: bool,
    // pub has_fluff_images: bool,
    // pub versions: Vec<RaceVersion>
}

impl Race {
    pub fn new(race: Value) -> Race {
        // Other Sources
        let mut other_sources: Vec<Source> = vec![];
        for other_source in race.get("otherSources").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).to_owned().iter() {
            other_sources.push(Source::new(other_source.get("source"), other_source.get("page")));
        }

        let name = race.get("name").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string();
        let source = Source::new(race.get("source"), race.get("page"));

        return Race {
            name: name.clone(),
            key: name.to_ascii_lowercase().replace(" ", "_") + "|" + source.name.clone().to_ascii_lowercase().as_str(),
            source,
            srd: race.get("srd").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
            basic_rules: race.get("basicRules").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
            other_sources,
            reprinted_as: race.get("reprintedAs").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            // copy: RaceCopy::new(race.get("_copy")),
            lineage: race.get("lineage").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            creature_types: race.get("creatureTypes").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            creature_type_tags: race.get("creatureTypeTags").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            size: race.get("size").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            speed: Speed::new(race.get("speed")),
            ability: Ability::new(race.get("ability")),
            height_and_weight: HeightAndWeight::new(race.get("heightAndWeight")),
            age: Age::new(race.get("age")),
            darkvision: race.get("darkvision").unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
            trait_tags: race.get("traitTags").unwrap_or(&to_value::<Vec<String>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            skill_proficiencies: SkillProficiencies::new(race.get("skillProficiencies")),
            language_proficiencies: LanguageProficiencies::new(race.get("languageProficiencies")),
            tool_proficiencies: ToolProficiencies::new(race.get("toolProficiencies")),
            weapon_proficiencies: WeaponProficiencies::new(race.get("weaponProficiencies")),
            armor_proficiencies: ArmorProficiencies::new(race.get("armorProficiencies")),
            resist: Resist::new(race.get("resist")),
            additional_spells: AdditionalSpells::new(race.get("additionalSpells")),
        };
    }
}

#[derive(Debug)]
pub struct Races {
    pub races: HashMap<String, Race>,
}

impl Races {
    pub fn new(races: Value) -> Races {
        let value_list: Vec<Value> = races.get("race").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).to_owned();
        // let mut race_list: Vec<Race> = vec![];
        println!("value_list length: {}", value_list.len());

        let mut num_of_na = 0;
        let mut race_map: HashMap<String, Race> = HashMap::new();
        for value in value_list {
            let new_race = Race::new(value);
            // race_list.push(new_race);
            if !new_race.key.contains("n/a") {
                race_map.insert(new_race.key.as_str().to_string(), new_race);
            } else {
                num_of_na += 1;
            }
        }
        println!("number of races with no name: {}", num_of_na);
        return Races { races: race_map };
    }
}
