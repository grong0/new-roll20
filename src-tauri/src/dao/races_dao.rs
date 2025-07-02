use std::{collections::HashMap, fs::read_to_string};

use serde_json::{from_str, to_value, Map, Value};

use crate::{dao::common::{Source, Speed}, serde_utils::{serde_as_array_mapping, serde_as_bool, serde_as_object_from_option, serde_as_string, serde_as_u64}};

use super::common::{form_key, Ability, AdditionalFeats, AdditionalSpells, Age, ArmorProficiencies, Entry, File, HeightAndWeight, LanguageProficiencies, Resist, SkillProficiencies, SkillToolLanguageProficiencies, ToolProficiencies, WeaponProficiencies};

use crate::serde_utils::{serde_as_array, serde_as_object};

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
struct Race {
    pub name: String,
    pub source: Source,
    pub key: String,
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
    pub additional_spells: Vec<AdditionalSpells>,
    pub immune: Vec<String>,
    pub condition_immune: Vec<String>,
    pub entries: Vec<Entry>,
    pub has_fluff: bool,
    pub has_fluff_images: bool,
    // pub versions: Vec<RaceVersion>
}

impl Race {
    pub fn new(object: Value) -> Race {
        let p_object = serde_as_object(&object, Map::new());

        let mut other_sources: Vec<Source> = vec![];
        for other_source in p_object.get("otherSources").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).to_owned().iter() {
            other_sources.push(Source::new(other_source.get("source"), other_source.get("page")));
        }

        let name = p_object.get("name").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string();
        let source = Source::new(p_object.get("source"), p_object.get("page"));

        return Race {
            key: form_key(&name, &source.name),
            name,
            source,
            srd: p_object.get("srd").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
            basic_rules: p_object.get("basicRules").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
            other_sources,
            reprinted_as: p_object.get("reprintedAs").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            lineage: p_object.get("lineage").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            creature_types: p_object.get("creatureTypes").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            creature_type_tags: p_object.get("creatureTypeTags").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            size: p_object.get("size").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            speed: Speed::new(p_object.get("speed")),
            ability: Ability::new(p_object.get("ability")),
            height_and_weight: HeightAndWeight::new(p_object.get("heightAndWeight")),
            age: Age::new(p_object.get("age")),
            darkvision: p_object.get("darkvision").unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
            trait_tags: p_object.get("traitTags").unwrap_or(&to_value::<Vec<String>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            skill_proficiencies: SkillProficiencies::new(p_object.get("skillProficiencies")),
            language_proficiencies: LanguageProficiencies::new(p_object.get("languageProficiencies")),
            tool_proficiencies: ToolProficiencies::new(p_object.get("toolProficiencies")),
            weapon_proficiencies: WeaponProficiencies::new(p_object.get("weaponProficiencies")),
            armor_proficiencies: ArmorProficiencies::new(p_object.get("armorProficiencies")),
            resist: Resist::new(p_object.get("resist")),
            additional_spells: serde_as_array(p_object.get("additionalSpells")).iter().map(|i| AdditionalSpells::new(Some(i))).collect(),
            immune: p_object.get("immune").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            condition_immune: p_object.get("conditionImmune").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
            entries: serde_as_array(p_object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
            has_fluff: p_object.get("hasFluff").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
            has_fluff_images: p_object.get("hasFluffImages").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
        };
    }
}

#[derive(Debug)]
struct Subrace {
    name: String,
    source: Source,
    key: String,
    race_name: String,
    reprinted_as: String,
    ability: Ability,
    entries: Vec<Entry>,
    has_fluff: bool,
    has_fluff_images: bool,
    skill_proficiencies: SkillProficiencies,
    srd: bool,
    darkvision: u64,
    resist: Vec<String>,
    overwrite: Vec<String>,
    other_sources: Vec<Source>,
    trait_tags: Vec<String>,
    language_proficiencies: LanguageProficiencies,
    additional_spells: AdditionalSpells,
    basic_rules: bool,
    height_and_weight: HeightAndWeight,
    armor_proficiencies: ArmorProficiencies,
    alias: Vec<String>,
    weapon_proficiencies: WeaponProficiencies,
    speed: Speed,
    skill_tool_language_proficiencies: SkillToolLanguageProficiencies,
    age: Age,
    tool_proficiencies: ToolProficiencies,
    sound_clip: File,
    feats: AdditionalFeats,
}

impl Subrace {
	pub fn new(value: Value) -> Subrace {
		let object = serde_as_object(&value, Map::new());

		let name = serde_as_string(object.get("name"), "N/A".to_string());
		let source = Source::new(object.get("source"), object.get("page"));

		return Subrace {
			key: form_key(&name, &source.name),
			name,
			source,
			race_name: serde_as_string(object.get("raceName"), "N/A".to_string()),
			reprinted_as: serde_as_string(object.get("raceName"), "N/A".to_string()),
			ability: Ability::new(object.get("ability")),
			entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
			has_fluff: serde_as_bool(object.get("hasFluff"), false),
			has_fluff_images: serde_as_bool(object.get("hasFluffImages"), false),
			skill_proficiencies: SkillProficiencies::new(object.get("skillProficiencies")),
			srd: serde_as_bool(object.get("srd"), false),
			darkvision: serde_as_u64(object.get("darkvision"), 0),
			resist: serde_as_array_mapping(object.get("resist"), serde_as_string, "N/A".to_string()),
			overwrite: serde_as_array_mapping(object.get("overwrite"), serde_as_string, "N/A".to_string()),
			other_sources: serde_as_array_mapping(object.get("otherSources"), serde_as_object_from_option, Map::new()).iter().map(|i| Source::new(i.get("source"), i.get("page"))).collect(),
			trait_tags: serde_as_array_mapping(object.get("trait_tags"), serde_as_string, "N/A".to_string()),
			language_proficiencies: LanguageProficiencies::new(object.get("languageProficiencies")),
			additional_spells: AdditionalSpells::new(object.get("additionalSpells")),
			basic_rules: serde_as_bool(object.get("basicRules"), false),
			height_and_weight: HeightAndWeight::new(object.get("heightAndWeight")),
			armor_proficiencies: ArmorProficiencies::new(object.get("armorProficiencies")),
			alias: serde_as_array_mapping(object.get("alias"), serde_as_string, "N/A".to_string()),
			weapon_proficiencies: WeaponProficiencies::new(object.get("weaponProficiencies")),
			speed: Speed::new(object.get("speed")),
			skill_tool_language_proficiencies: SkillToolLanguageProficiencies::new(object.get("skillToolLanguageProficiencies")),
			age: Age::new(object.get("age")),
			tool_proficiencies: ToolProficiencies::new(object.get("toolProficiencies")),
			sound_clip: File::new(object.get("soundClip")),
			feats: AdditionalFeats::new(object.get("additionalFeats")),
		}
	}
}

#[derive(Debug)]
pub struct Races {
    pub races: HashMap<String, Race>,
	pub subraces: HashMap<String, Subrace>>
}

impl Races {
    pub fn new(path: &str) -> Races {
        let file = read_to_string(path);
        let mut races = HashMap::new();
		let mut subraces = HashMap::new();
        if file.is_err() {
            return Races { races, subraces };
        }
        let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
        let races_value_list: Vec<Value> = serde_as_array(serde_file.get("race"));
		let subraces_value_list: Vec<Value> = serde_as_array(serde_file.get("subrace"));
        println!("num of races: {}", races_value_list.len());
		println!("num of subraces: {}", subraces_value_list.len());

        let mut num_of_na = 0;
        for value in races_value_list {
            let new_struct = Race::new(value);
            if !new_struct.key.contains("n/a") {
                races.insert(new_struct.key.as_str().to_string(), new_struct);
            } else {
                num_of_na += 1;
            }
        }
        println!("number of races with no name: {}", num_of_na);
		num_of_na = 0;
		for value in subraces_value_list {
			let new_struct = Subrace::new(value);
			if !new_struct.key.contains("n/a") {
				subraces.insert(new_struct.key.as_str().to_string(), new_struct);
			} else {
				num_of_na += 1;
			}
		}
		println!("number of subraces with no name: {}", num_of_na);

        return Races { races, subraces };
    }

    pub fn get_races_by_name(self, name: String) -> Vec<Race> {
        let mut matches = vec![];
        for (_, race) in self.races {
            if race.name.to_lowercase().contains(name.to_lowercase().as_str()) {
                matches.push(race);
            }
        }
        return matches;
    }

    pub fn get_unique_race_names(self) -> Vec<String> {
        let mut names = vec![];
        for (_, race) in self.races {
            if !names.contains(&race.name) {
                names.push(race.name.clone());
            }
        }
        return names;
    }
}
