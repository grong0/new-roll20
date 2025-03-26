use serde_json::{to_value, Value};

use crate::dao::misc_classes::Source;

#[derive(Debug)]
pub struct ModifyRaceCopy {
    mode: String,
    name: String,
    entries: Vec<String>,
}

impl ModifyRaceCopy {
    pub fn new(entry: &Value) -> ModifyRaceCopy {
        let mut entries: Vec<String> = vec![];
        for entry in entry
            .get("items")
            .unwrap()
            .get("entries")
            .unwrap_or(&to_value::<Vec<String>>(vec![]).unwrap())
            .as_array()
            .unwrap_or(&vec![])
            .iter()
        {
            entries.push(entry.as_str().unwrap_or("N/A").to_string());
        }
        return ModifyRaceCopy {
            mode: entry
                .get("mode")
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            name: entry
                .get("items")
                .unwrap()
                .get("name")
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
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
        for modify in copy
            .unwrap()
            .get("_mod")
            .unwrap()
            .get("entries")
            .unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap())
            .as_array()
            .unwrap_or(&vec![])
            .iter()
        {
            modifying.push(ModifyRaceCopy::new(modify))
        }
        let mut preserving: Vec<String> = vec![];
        for modify in copy
            .unwrap()
            .get("_preserve")
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .into_iter()
        {
            preserving.push(modify.to_owned());
        }
        return RaceCopy {
            name: copy
                .unwrap()
                .get("name")
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            source: copy
                .unwrap()
                .get("source")
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            modifying,
            preserving,
        };
    }
}

#[derive(Debug)]
pub struct Race {
    pub name: String,
    pub source: Source,
    pub srd: bool,
    pub basic_rules: bool,
    pub other_sources: Vec<Source>,
    pub reprinted_as: Vec<String>,
    // pub copy: RaceCopy,
    pub lineage: String,
    // pub creature_types: Vec<String>,
    // pub creature_type_tags: Vec<String>,
    // pub size: Vec<String>,
    // pub speed: Speed,
    // pub ability: Ability,
    // pub height_and_weight: HeightAndWeight,
    // pub age: Age,
    // pub darkvision: u8,
    // pub trait_tags: Vec<String>,
    // pub skill_proficiencies: SkillProficiencies,
    // pub language_proficiencies: LanguageProficiencies,
    // pub tool_proficiencies: ToolProficiencies,
    // pub weapon_proficiencies: WeaponProficiencies,
    // pub armor_proficiencies: ArmorProficiencies,
    // pub resist: Resist,
    // pub additional_spells: AdditionalSpells,
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
        for other_source in race
            .get("otherSources")
            .unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap())
            .as_array()
            .unwrap_or(&vec![])
            .to_owned()
            .iter()
        {
            other_sources.push(Source::new(
                other_source.get("source"),
                other_source.get("page"),
            ));
        }

        // Reprinted As
        let mut reprinted_as: Vec<String> = vec![];
        for reprint in race
            .get("reprintedAs")
            .unwrap_or(&to_value::<Vec<String>>(vec![]).unwrap())
            .as_array()
            .unwrap_or(&vec![])
        {
            reprinted_as.push(reprint.as_str().unwrap_or("").to_string());
        }

        return Race {
            name: race
                .get("name")
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            source: Source::new(race.get("source"), race.get("page")),
            srd: race
                .get("srd")
                .unwrap_or(&to_value(false).unwrap())
                .as_bool()
                .unwrap_or(false),
            basic_rules: race
                .get("basicRules")
                .unwrap_or(&to_value(false).unwrap())
                .as_bool()
                .unwrap_or(false),
            other_sources,
            reprinted_as,
            // copy: RaceCopy::new(race.get("_copy")),
            lineage: race
                .get("lineage")
                .unwrap_or(&to_value("N/A").unwrap())
                .to_string(),
        };
    }
}

#[derive(Debug)]
pub struct Races {
    pub races: Vec<Race>,
}

impl Races {
    pub fn new(races: Value) -> Races {
        let value_list: Vec<Value> = races
            .get("race")
            .unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap())
            .as_array()
            .unwrap_or(&vec![])
            .to_owned();
        let mut race_list: Vec<Race> = vec![];
        println!("value_list length: {}", value_list.len());
        for value in value_list {
            race_list.push(Race::new(value));
        }
        return Races { races: race_list };
    }
}
