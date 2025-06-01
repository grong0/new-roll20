use serde_json::{to_value, Map, Value};

use crate::dao::common::{Source, Speed};

use super::common::{form_key, serde_as_array, serde_as_object, Ability, AdditionalSpells, Age, ArmorProficiencies, Entry, HeightAndWeight, LanguageProficiencies, Resist, SkillProficiencies, ToolProficiencies, WeaponProficiencies};

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
    pub additional_spells: AdditionalSpells,
    pub immune: Vec<String>,
    pub condition_immune: Vec<String>,
    pub entries: Vec<Entry>,
    pub has_fluff: bool,
    pub has_fluff_images: bool,
    // pub versions: Vec<RaceVersion>
}

impl Race {
    pub fn new(object: Option<&Value>) -> Race {
		let p_object = serde_as_object(object, Map::new());

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
            // copy: RaceCopy::new(p_object.get("_copy")),
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
            additional_spells: AdditionalSpells::new(p_object.get("additionalSpells")),
			immune: p_object.get("immune").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
			condition_immune: p_object.get("conditionImmune").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect(),
			entries: serde_as_array(p_object.get("entries")).iter().map(|i| Entry::new(Some(i))).collect(),
			has_fluff: p_object.get("hasFluff").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
			has_fluff_images: p_object.get("hasFluffImages").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
        };
    }
}
