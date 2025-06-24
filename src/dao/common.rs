use std::{
    collections::HashMap,
    iter::{zip, Enumerate},
};

use serde_json::{to_value, Map, Value};

use super::items_dao::Item;

pub fn form_key(name: &String, source: &String) -> String {
    return name.to_ascii_lowercase().replace(" ", "_") + "|" + source.to_ascii_lowercase().as_str();
}

pub fn serde_as_string(value: Option<&Value>, default: String) -> String {
    return value.unwrap_or(&to_value(&default).unwrap()).as_str().unwrap_or(&default.to_string()).to_string();
}

pub fn serde_as_u64(value: Option<&Value>, default: u64) -> u64 {
    return value.unwrap_or(&to_value(default).unwrap()).as_u64().unwrap_or(default);
}

pub fn serde_as_i64(value: Option<&Value>, default: i64) -> i64 {
    return value.unwrap_or(&to_value(default).unwrap()).as_i64().unwrap_or(default);
}

pub fn serde_as_bool(value: Option<&Value>, default: bool) -> bool {
    return value.unwrap_or(&to_value(default).unwrap()).as_bool().unwrap_or(default);
}

pub fn serde_as_object(value: &Value, default: Map<String, Value>) -> Map<String, Value> {
    return value.as_object().unwrap_or(&default.clone()).to_owned();
}

pub fn serde_as_object_from_option(value: Option<&Value>, default: Map<String, Value>) -> Map<String, Value> {
    return value.unwrap_or(&to_value::<Map<String, Value>>(default.clone()).unwrap()).as_object().unwrap_or(&default.clone()).to_owned();
}

pub fn serde_as_array(value: Option<&Value>) -> Vec<Value> {
    return value.unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).to_owned();
}

pub fn serde_as_array_mapping<T: Clone>(value: Option<&Value>, mapping_func: fn(Option<&Value>, T) -> T, default: T) -> Vec<T> {
    return value.unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|i| mapping_func(Some(i), default.clone())).collect();
}

#[derive(Debug)]
pub struct Speed {
    walk: u64,
    fly: u64,
    swim: u64,
    climb: u64,
}

impl Speed {
    pub fn new(object: Option<&Value>) -> Speed {
        let mut walk = 30;
        let mut fly = 0;
        let mut swim = 0;
        let mut climb = 0;

        if object.is_some() {
            match object.unwrap().as_object() {
                Some(new_object) => {
                    walk = new_object.get("walk").unwrap_or(&to_value(walk).unwrap()).as_u64().unwrap_or(walk);
                    let default_fly_val = to_value(fly).unwrap();
                    let fly_val = new_object.get("fly").unwrap_or(&default_fly_val);
                    match fly_val.as_u64() {
                        Some(fly_int) => fly = fly_int,
                        None => fly = if fly_val.as_bool().unwrap_or(false) { walk } else { fly },
                    }
                    let default_swim_val = to_value(swim).unwrap();
                    let swim_val = new_object.get("swim").unwrap_or(&default_swim_val);
                    match swim_val.as_u64() {
                        Some(swim_int) => swim = swim_int,
                        None => swim = if swim_val.as_bool().unwrap_or(false) { walk } else { swim },
                    }
                    let default_climb_val = to_value(climb).unwrap();
                    let climb_val = new_object.get("climb").unwrap_or(&default_climb_val);
                    match climb_val.as_u64() {
                        Some(climb_int) => climb = climb_int,
                        None => climb = if climb_val.as_bool().unwrap_or(false) { walk } else { climb },
                    }
                }
                None => {
                    walk = object.unwrap().as_u64().unwrap_or(walk);
                }
            }
        }

        return Speed { walk, fly, swim, climb };
    }
}

#[derive(Debug)]
pub struct ChooseAbility {
    count: u64,
    abilities: Vec<String>,
}

impl ChooseAbility {
    pub fn new(object: Option<&Value>) -> ChooseAbility {
        let mut count = 0;
        let mut abilities = vec![];

        if object.is_some() {
            if object.unwrap().is_object() && object.unwrap().as_object().unwrap().get("from").is_some() && object.unwrap().as_object().unwrap().get("count").is_some() {
                let parsed_object = object.unwrap().as_object().unwrap();
                count = parsed_object.get("count").unwrap_or(&to_value(count).unwrap()).as_u64().unwrap_or(count);
                abilities = parsed_object.get("from").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect();
            } else if object.unwrap().is_array() {
                let parsed_array = object.unwrap().as_array().unwrap();
                count = 1;
                abilities = parsed_array.iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect();
            }
        }

        return ChooseAbility { count, abilities };
    }
}

#[derive(Debug)]
pub enum Abilities {
	STRENGTH,
	DEXTERITY,
	CONSTITUTION,
	INTELLIGENCE,
	WISDOM,
	CHARISMA,
	SPELL,
	NONE,
}

#[derive(Debug)]
pub struct Ability {
    strength: i64,
    dexterity: i64,
    constitution: i64,
    intelligence: i64,
    wisdom: i64,
    charisma: i64,
    choose: ChooseAbility,
}

impl Ability {
    pub fn new(value: Option<&Value>) -> Ability {
        let object: Map<String, Value>;

        if value.is_some() {
            let value_unwrapped = value.unwrap();

            if value_unwrapped.is_object() {
                object = value_unwrapped.as_object().unwrap().to_owned();
            } else if value_unwrapped.is_array() {
                object = serde_as_object_from_option(value_unwrapped.as_array().unwrap().get(0), Map::new());
            } else {
                println!("ability value was something other than an object or an array: {:?}", value.unwrap().to_string());
                object = Map::new();
            }
        } else {
            object = Map::new();
        }

        return Ability {
            strength: serde_as_i64(object.get("str"), 0),
            dexterity: serde_as_i64(object.get("dex"), 0),
            constitution: serde_as_i64(object.get("con"), 0),
            intelligence: serde_as_i64(object.get("int"), 0),
            wisdom: serde_as_i64(object.get("wis"), 0),
            charisma: serde_as_i64(object.get("cha"), 0),
            choose: ChooseAbility::new(object.get("choose")),
        };
    }
}

#[derive(Debug)]
pub struct Source {
    pub name: String,
    pub page: u64,
}

impl Source {
    pub fn new(source: Option<&Value>, page: Option<&Value>) -> Source {
        return Source {
            name: source.unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            page: page.unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
        };
    }
}

#[derive(Debug)]
pub struct HeightAndWeight {
    base_height: u64,
    height_mod: String,
    base_weight: u64,
    weight_mod: String,
}

impl HeightAndWeight {
    pub fn new(object: Option<&Value>) -> HeightAndWeight {
        let mut base_height = 0;
        let mut height_mod = "N/A".to_string();
        let mut base_weight = 0;
        let mut weight_mod = "N/A".to_string();

        if object.is_some() && object.unwrap().is_object() {
            let parsed_object = object.unwrap().as_object().unwrap();
            base_height = serde_as_u64(parsed_object.get("baseHeight"), base_height);
            height_mod = serde_as_string(parsed_object.get("heightMod"), height_mod);
            base_weight = serde_as_u64(parsed_object.get("baseWeight"), base_weight);
            weight_mod = serde_as_string(parsed_object.get("weightMod"), weight_mod);
        }

        return HeightAndWeight { base_height, height_mod, base_weight, weight_mod };
    }
}

#[derive(Debug)]
pub struct Age {
    mature: u64,
    max: u64,
}

impl Age {
    pub fn new(object: Option<&Value>) -> Age {
        let mut mature = 0;
        let mut max = 0;

        if object.is_some() && object.unwrap().is_object() {
            let parsed_object = object.unwrap().as_object().unwrap();
            mature = parsed_object.get("mature").unwrap_or(&to_value(mature).unwrap()).as_u64().unwrap_or(mature);
            max = parsed_object.get("max").unwrap_or(&to_value(max).unwrap()).as_u64().unwrap_or(max);
        }

        return Age { mature, max };
    }
}

#[derive(Debug)]
pub struct SkillProficiencies {
    skills: Vec<String>,
    any: u64,
    choose_skills: Vec<String>,
    choose_count: u64,
}

impl SkillProficiencies {
    pub fn new(object: Option<&Value>) -> SkillProficiencies {
        let mut skills: Vec<String> = vec![];
        let mut any = 0;
        let mut choose_skills: Vec<String> = vec![];
        let mut choose_count = 0;

        if object.is_some() {
            let parsed_object: Map<String, Value>;
            if object.unwrap().is_object() {
                parsed_object = serde_as_object_from_option(object, Map::new()).to_owned();
            } else if object.unwrap().is_array() {
                parsed_object = serde_as_array_mapping(object, serde_as_object_from_option, Map::new()).get(0).unwrap_or(&Map::new()).to_owned();
            } else {
                parsed_object = Map::new();
            }
            for (key, value) in parsed_object {
                if key == "any" {
                    any = value.as_u64().unwrap();
                } else if key == "choose" {
                    let choose = value.as_object().unwrap();
                    choose_skills = serde_as_array_mapping(choose.get("from"), serde_as_string, "N/A".to_string());
                    choose_count = serde_as_u64(choose.get("count"), 1);
                } else {
                    skills.push(key.to_string());
                }
            }
        }

        return SkillProficiencies { skills, any, choose_skills, choose_count };
    }
}

#[derive(Debug)]
pub struct LanguageProficiencies {
    languages: Vec<String>,
    any_standard: u64,
    choose_languages: Vec<String>,
    choose_count: u64,
}

// TODO: add support for getting the 'other' language from entries
impl LanguageProficiencies {
    pub fn new(object: Option<&Value>) -> LanguageProficiencies {
        let mut languages: Vec<String> = vec![];
        let mut any_standard = 0;
        let mut choose_languages: Vec<String> = vec![];
        let mut choose_count = 0;

        if object.is_some() {
            let parsed_object: Map<String, Value>;
            if object.unwrap().is_object() {
                parsed_object = object.unwrap().as_object().unwrap().to_owned();
            } else if object.unwrap().is_array() {
                parsed_object = object.unwrap().as_array().unwrap().get(0).unwrap_or(&to_value(Map::new()).unwrap()).as_object().unwrap_or(&Map::new()).to_owned();
            } else {
                parsed_object = Map::new();
            }
            for (key, value) in parsed_object {
                if key == "anyStandard" {
                    any_standard = value.as_u64().unwrap();
                } else if key == "choose" {
                    let choose = value.as_object().unwrap();
                    choose_languages = choose.get("from").unwrap().as_array().unwrap().iter().map(|lan| lan.to_string()).collect();
                    choose_count = choose.get("count").unwrap_or(&to_value(1).unwrap()).as_u64().unwrap_or(1);
                } else {
                    languages.push(key.to_string());
                }
            }
        }

        return LanguageProficiencies {
            languages,
            any_standard,
            choose_languages,
            choose_count,
        };
    }
}

#[derive(Debug)]
pub struct ToolProficiencies {
    tools: Vec<String>,
    choose_any_amount: u64,
}

impl ToolProficiencies {
    pub fn new(object: Option<&Value>) -> ToolProficiencies {
        let mut tools: Vec<String> = vec![];
        let mut choose_any_amount = 0;

        if object.is_some() {
            let parsed_object: Map<String, Value>;
            if object.unwrap().is_object() {
                parsed_object = object.unwrap().as_object().unwrap().to_owned();
            } else if object.unwrap().is_array() {
                parsed_object = object.unwrap().as_array().unwrap().get(0).unwrap_or(&to_value(Map::new()).unwrap()).as_object().unwrap_or(&Map::new()).to_owned();
            } else {
                parsed_object = Map::new();
            }
            for (key, value) in parsed_object {
                if key == "any" {
                    choose_any_amount = value.as_u64().unwrap()
                } else {
                    tools.push(key.to_string());
                }
            }
        }

        return ToolProficiencies { tools, choose_any_amount };
    }
}

#[derive(Debug)]
pub struct WeaponProficiencies {
    weapons: Vec<String>,
    choose_filter: String, // Only from one race: Hobgoblin|VGM
    choose_amount: u64,
}

impl WeaponProficiencies {
    pub fn new(value: Option<&Value>) -> WeaponProficiencies {
        let mut weapons: Vec<String> = vec![];
        let mut choose_filter = "N/A".to_string();
        let mut choose_amount = 0;

        let array = serde_as_array(value);

        for item in array {
            if item.is_object() {
                for (key, value) in item.as_object().unwrap() {
                    if key == "choose" {
                        let choose = value.as_object().unwrap();
                        choose_filter = choose.get("fromFilter").unwrap_or(&to_value(&choose_filter).unwrap()).as_str().unwrap_or(&choose_filter).to_string();
                        choose_amount = choose.get("count").unwrap_or(&to_value(choose_amount).unwrap()).as_u64().unwrap_or(choose_amount);
                    } else {
                        weapons.push(key.to_string());
                    }
                }
            } else if item.is_string() {
                weapons.push(item.as_str().unwrap().to_string());
            } else {
                println!("item in weapon proficiencies was something other than a string or an object");
            }
        }

        // if value.is_some() {
        // 	let value_unwrapped = value.unwrap();
        //     let object: Map<String, Value>;
        //     if value_unwrapped.is_object() {
        //         object = serde_as_object(value_unwrapped, Map::new());
        //     } else if value_unwrapped.is_array() {
        // 		object = serde_as_array_mapping(value, serde_as_object_from_option, Map::new()).get(0).unwrap_or(&Map::new()).to_owned();
        //     } else {
        // 		object = Map::new();
        //     }
        //     for (key, value) in object {
        //         if key == "choose" {
        //             let choose = value.as_object().unwrap();
        //             choose_filter = choose.get("fromFilter").unwrap_or(&to_value(&choose_filter).unwrap()).as_str().unwrap_or(&choose_filter).to_string();
        //             choose_amount = choose.get("count").unwrap_or(&to_value(choose_amount).unwrap()).as_u64().unwrap_or(choose_amount);
        //         } else {
        //             weapons.push(key.to_string());
        //         }
        //     }
        // }

        return WeaponProficiencies { weapons, choose_filter, choose_amount };
    }
}

#[derive(Debug)]
pub struct ArmorProficiencies {
    armor: Vec<String>,
}

impl ArmorProficiencies {
    pub fn new(object: Option<&Value>) -> ArmorProficiencies {
        let mut armor: Vec<String> = vec![];

        if object.is_some() {
            let parsed_object: Map<String, Value>;
            if object.unwrap().is_object() {
                parsed_object = object.unwrap().as_object().unwrap().to_owned();
            } else if object.unwrap().is_array() {
                parsed_object = object.unwrap().as_array().unwrap().get(0).unwrap_or(&to_value(Map::new()).unwrap()).as_object().unwrap_or(&Map::new()).to_owned();
            } else {
                parsed_object = Map::new();
            }
            armor = parsed_object.keys().into_iter().map(|arm| arm.to_string()).collect();
        }

        return ArmorProficiencies { armor };
    }
}

#[derive(Debug)]
pub struct Resist {
    resistances: Vec<String>,
    choose_from: Vec<String>,
}

impl Resist {
    pub fn new(list: Option<&Value>) -> Resist {
        let mut resistances: Vec<String> = vec![];
        let mut choose_from: Vec<String> = vec![];

        if list.is_some() && list.unwrap().is_array() {
            for item in list.unwrap().as_array().unwrap() {
                if item.is_object() {
                    choose_from = item.as_object().unwrap().get("from").unwrap_or(&to_value::<Vec<String>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|res| res.to_string()).collect();
                } else {
                    resistances.push(item.as_str().unwrap_or("").to_string());
                }
            }
        }

        return Resist { resistances, choose_from };
    }
}

#[derive(Clone, Debug)]
pub enum Recharge {
    REST,     // Spells which recharge on short or long rest
    DAILY,    // Spells which recharge on long rest
    RESOURCE, // Spells which cost a specific resource to use (such as Ki), but otherwise have no restrictions
    LIMITED,  // Spells which do not recharge, and have a limited number of uses
    NEITHER,
}

impl Recharge {
    pub fn new(value: &String) -> Recharge {
        return match value.as_str() {
            "rest" => Recharge::REST,
            "daily" => Recharge::DAILY,
            "resource" => Recharge::RESOURCE,
            "limited" => Recharge::LIMITED,
            _ => Recharge::NEITHER,
        };
    }
}

#[derive(Clone, Debug)]
pub enum AdditionalSpellCategory {
    WILL,
    RITUAL,
    NEITHER,
}

impl AdditionalSpellCategory {
    pub fn new(value: &String) -> AdditionalSpellCategory {
        return match value.as_str() {
            "will" => AdditionalSpellCategory::WILL,
            "ritual" => AdditionalSpellCategory::RITUAL,
            _ => AdditionalSpellCategory::NEITHER,
        };
    }
}

#[derive(Debug)]
pub struct AdditionalSpell {
    filter: String,
    count: u64,
    aquired_at: String,
    recharge: Recharge,
    category: AdditionalSpellCategory,
}

impl AdditionalSpell {
    fn parse_array_of_string_or_filter_object(value: &Value, aquired_at: &String, recharge: Recharge, category: AdditionalSpellCategory) -> Vec<AdditionalSpell> {
        let mut array: Vec<AdditionalSpell> = vec![];
        for item in serde_as_array(Some(&value)) {
            let mut filter = "N/A".to_string();
            let mut count = 1;
            if item.is_string() {
                filter = item.as_str().unwrap().to_string();
            } else if item.is_object() {
                let object = item.as_object().unwrap();
                if object.contains_key("all") {
                    filter = serde_as_string(object.get("all"), "N/A".to_string());
                } else if object.contains_key("choose") {
                    filter = serde_as_string(object.get("choose"), "N/A".to_string());
                    count = serde_as_u64(object.get("count"), 1);
                } else {
                    println!("no expected keys present in string_or_filter_object under additional spells");
                }
            }
            array.push(AdditionalSpell {
                filter,
                count,
                aquired_at: aquired_at.clone(),
                recharge: recharge.clone(),
                category: category.clone(),
            });
        }

        return array;
    }

    pub fn find_additional_spells(value: Option<&Value>) -> Vec<AdditionalSpell> {
        let object = serde_as_object_from_option(value, Map::new());

        let mut additional_spells = vec![];
        for (key, value) in object {
            let aquired_at = key.as_str().to_string();
            let object = serde_as_object(&value, Map::new());
            if value.is_object() && (!object.contains_key("choose") || object.contains_key("all")) {
                for (key, value) in object {
                    if vec!["rest", "daily", "resource", "limited"].contains(&key.as_str()) {
                        let recharge = Recharge::new(&key);
                        for (_, value) in serde_as_object(&value, Map::new()) {
                            additional_spells.extend(AdditionalSpell::parse_array_of_string_or_filter_object(&value, &aquired_at, recharge.clone(), AdditionalSpellCategory::NEITHER));
                        }
                    } else {
                        additional_spells.extend(AdditionalSpell::parse_array_of_string_or_filter_object(&value, &aquired_at, Recharge::NEITHER, AdditionalSpellCategory::new(&key)));
                    }
                }
            } else {
                additional_spells.extend(AdditionalSpell::parse_array_of_string_or_filter_object(&value, &aquired_at, Recharge::NEITHER, AdditionalSpellCategory::NEITHER));
            }
        }

        return additional_spells;
    }
}

/// There are 5 keys:
/// 1. known | Spells which are always known
/// 2. innate | Spells which can be innately cast, without expending normal spell resources
/// 3. expanded | Expansions to a class' default spell list, from which spells can be chosen (e.g. Warlock Patron Spells)
/// 4. ability | Optionally specify the ability score used for e.g. racial spellcasting
/// 5. prepared | Spells which are always prepared
/// 6. resourceName | Optional resource name for resource-cast spells in this group
/// 7. name | Optional display name for the group
///
/// _additionalSpellArrayOfStringOrFilterObject or _additionalSpellLevelObject
///
/// TODO: Finish this
#[derive(Debug)]
pub struct AdditionalSpells {
    name: String,
    ability_choices: Vec<String>,
    resource_name: String,
    innate: Vec<AdditionalSpell>,
    known: Vec<AdditionalSpell>,
    prepared: Vec<AdditionalSpell>,
    expanded: Vec<AdditionalSpell>,
}

impl AdditionalSpells {
    pub fn new(value: Option<&Value>) -> AdditionalSpells {
        let object = serde_as_object_from_option(value, Map::new());

        let ability_choices: Vec<String>;
        if object.contains_key("ability") {
            let ability = object.get("ability").unwrap();
            if ability.is_string() {
                ability_choices = vec![ability.as_str().unwrap().to_string()];
            } else if ability.is_object() {
                ability_choices = serde_as_array_mapping(ability.as_object().unwrap().get("choose"), serde_as_string, "N/A".to_string());
            } else {
                ability_choices = vec![];
                println!("unexpected value type in additional spells ability");
            }
        } else {
            ability_choices = vec![];
        }

        return AdditionalSpells {
            name: serde_as_string(object.get("name"), "N/A".to_string()),
            ability_choices,
            resource_name: serde_as_string(object.get("resourceName"), "N/A".to_string()),
            innate: AdditionalSpell::find_additional_spells(object.get("innate")),
            known: AdditionalSpell::find_additional_spells(object.get("known")),
            prepared: AdditionalSpell::find_additional_spells(object.get("prepared")),
            expanded: AdditionalSpell::find_additional_spells(object.get("expanded")),
        };
    }
}

#[derive(Debug)]
pub struct Time {
    quantity: u64,
    unit: String,
}

impl Time {
    pub fn new(object: Option<&Value>) -> Time {
        let p_object = serde_as_object_from_option(object, Map::new());
        return Time {
            quantity: serde_as_u64(p_object.get("number"), 0),
            unit: serde_as_string(p_object.get("unit"), "N/A".to_string()),
        };
    }
}

#[derive(Debug)]
pub struct Range {
    typer: String,
    unit: String,
    amount: u64,
}

impl Range {
    pub fn new(value: Option<&Value>) -> Range {
        let object = serde_as_object_from_option(value, Map::new());
        let distance = serde_as_object_from_option(object.get("distance"), Map::new());

        return Range {
            typer: serde_as_string(object.get("type"), "N/A".to_string()),
            unit: distance.get("type").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            amount: distance.get("amount").unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
        };
    }
}

#[derive(Debug)]
pub struct Components {
    v: bool,
    s: bool,
    m: String,
    r: String,
}

impl Components {
    pub fn new(object: Option<&Value>) -> Components {
        let parsed_object = serde_as_object_from_option(object, Map::new());

        return Components {
            v: serde_as_bool(parsed_object.get("v"), false),
            s: serde_as_bool(parsed_object.get("s"), false),
            m: serde_as_string(parsed_object.get("m"), "N/A".to_string()),
            r: serde_as_string(parsed_object.get("r"), "N/A".to_string()),
        };
    }
}

#[derive(Debug)]
pub struct Duration {
    typed: String,
    unit: String,
    amount: u64,
    concentration: bool,
    ends: Vec<String>,
}

impl Duration {
    pub fn new(object: Option<&Value>) -> Duration {
        let parsed_object = serde_as_object_from_option(object, Map::new());
        let mut ends = vec![];

        for end in serde_as_array(parsed_object.get("ends")) {
            ends.push(end.as_str().unwrap_or("N/A").to_string());
        }

        let mut unit: String = "N/A".to_string();
        let mut amount: u64 = 0;

        match parsed_object.get("duration") {
            Some(duration) => {
                unit = duration.get("type").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string();
                amount = duration.get("amount").unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0);
            }
            None => {}
        }

        return Duration {
            typed: parsed_object.get("type").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            unit,
            amount,
            concentration: parsed_object.get("concentration").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
            ends,
        };
    }
}

#[derive(Debug)]
pub struct LevelDie {
    level: String,
    die: String,
}

#[derive(Debug)]
pub struct ScalingLevelDice {
    label: String,
    scaling: Vec<LevelDie>,
}

impl ScalingLevelDice {
    pub fn new(object: Option<&Value>) -> ScalingLevelDice {
        let parsed_object = serde_as_object_from_option(object, Map::new());

        let mut scaling = vec![];
        let scaling_object = serde_as_object_from_option(parsed_object.get("scaling"), Map::new());
        for level in scaling_object.keys() {
            scaling.push(LevelDie {
                level: level.to_string(),
                die: scaling_object.get(level).unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            });
        }

        return ScalingLevelDice {
            label: parsed_object.get("label").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            scaling,
        };
    }
}

#[derive(Debug)]
pub struct PackItem {
    name: String,
    quantity: u64,
    special: bool,
}

impl PackItem {
    pub fn new(item: &Value) -> PackItem {
        let object = serde_as_object(&item, Map::new());

        let name: String;
        match object.get("special") {
            Some(special) => {
                name = special.as_str().unwrap_or("NOT_STRING_SPECIAL").to_string();
            }
            None => {
                name = serde_as_string(object.get("item"), "N/A".to_string());
            }
        }

        return PackItem {
            name,
            quantity: object.get("quantity").unwrap_or(&to_value(1).unwrap()).as_u64().unwrap_or(1),
            special: false,
        };
    }
}

#[derive(Debug)]
pub enum ItemType {
    Firearm,
    Arrow,
    Axe,
    Armor,
    Club,
    Bolt,
    Dagger,
    Sword,
    Polearm,
    Crossbow,
    Spear,
    Hammer,
    Bow,
    Mace,
    Net,
    Staff,
    BulletSling,
    Neither,
}

impl ItemType {
    pub fn new(item_type: Option<&Value>) -> ItemType {
        match item_type.unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A") {
            "firearm" => return ItemType::Firearm,
            "arrow" => return ItemType::Arrow,
            "axe" => return ItemType::Axe,
            "armor" => return ItemType::Armor,
            "club" => return ItemType::Club,
            "bolt" => return ItemType::Bolt,
            "dagger" => return ItemType::Dagger,
            "sword" => return ItemType::Sword,
            "polearm" => return ItemType::Polearm,
            "crossbow" => return ItemType::Crossbow,
            "spear" => return ItemType::Spear,
            "hammer" => return ItemType::Hammer,
            "bow" => return ItemType::Bow,
            "mace" => return ItemType::Mace,
            "net" => return ItemType::Net,
            "staff" => return ItemType::Staff,
            "bulletSling" => return ItemType::BulletSling,
            _ => return ItemType::Neither,
        }
    }
}

#[derive(Debug)]
pub struct ValidContainerSlotItem {
    name: String,
    quantity: u64,
}

impl ValidContainerSlotItem {
    pub fn new(name: String, quantity: Value) -> ValidContainerSlotItem {
        return ValidContainerSlotItem {
            name: name.to_string(),
            quantity: quantity.as_u64().unwrap_or(1),
        };
    }
}

#[derive(Debug)]
pub struct ContainerSlot {
    weight_limit: u64,
    valid_items: Vec<ValidContainerSlotItem>,
}

impl ContainerSlot {
    pub fn new(weight_limit: u64, items_object: Map<String, Value>) -> ContainerSlot {
        let mut valid_items: Vec<ValidContainerSlotItem> = vec![];
        for (key, value) in items_object {
            valid_items.push(ValidContainerSlotItem::new(key, value));
        }

        return ContainerSlot { weight_limit, valid_items };
    }
}

#[derive(Debug)]
pub struct ContainerCapacity {
    slots: Vec<ContainerSlot>,
    weightless: bool,
}

impl ContainerCapacity {
    pub fn new(object: Option<&Value>) -> ContainerCapacity {
        let parsed_object = serde_as_object_from_option(object, Map::new());

        // Parses weight and item specifications
        let mut parsed_weights: Vec<u64> = vec![];
        for weight in serde_as_array(parsed_object.get("weight")) {
            parsed_weights.push(weight.as_u64().unwrap_or(0));
        }
        let mut parsed_items: Vec<Map<String, Value>> = vec![];
        for item in serde_as_array(parsed_object.get("item")) {
            parsed_items.push(serde_as_object(&item, Map::new()));
        }

        // Extends the smallest list to be the same size as the other
        let parsed_weights_len = parsed_weights.len();
        let parsed_items_len = parsed_items.len();
        if parsed_weights_len < parsed_items_len {
            let extension: Vec<u64> = (0..parsed_items_len - parsed_weights_len).map(|_| 0).collect();
            parsed_weights.extend(extension);
        } else if parsed_items_len < parsed_weights_len {
            for _ in 0..parsed_weights_len - parsed_items_len {
                parsed_items.push(Map::new());
            }
        }

        // Collects populated container slots
        let mut slots = vec![];
        for (weight, item) in zip(parsed_weights, parsed_items) {
            slots.push(ContainerSlot::new(weight, item));
        }

        return ContainerCapacity {
            slots,
            weightless: parsed_object.get("weightless").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
        };
    }
}

#[derive(Debug)]
pub struct StartingItem {
    name: String,
    quantity: u64,
    value: u64,
    contains_value: u64,
    display_name: String,
    is_pouch: bool,
    is_special: bool,
    is_equipment_type: bool,
    is_money: bool,
}

impl StartingItem {
    pub fn new(item: Value) -> StartingItem {
        let mut name = "N/A".to_string();
        let mut quantity = 1;
        let mut value = 0;
        let mut contains_value = 0;
        let mut display_name = "".to_string();
        let mut is_pouch = false;
        let mut is_special = false;
        let mut is_equipment_type = false;
        let mut is_money = false;

        if item.as_object().is_some() {
            let object = item.as_object().unwrap();
            for (key, key_value) in object {
                match key.as_str() {
                    "item" => {
                        name = key_value.as_str().unwrap_or(&name).to_string();
                    }
                    "special" => {
                        name = key_value.as_str().unwrap_or(&name).to_string();
                        is_special = true;
                    }
                    "worthValue" => {
                        value = key_value.as_u64().unwrap_or(value);
                    }
                    "containsValue" => {
                        is_pouch = true;
                        contains_value = key_value.as_u64().unwrap_or(contains_value);
                    }
                    "quantity" => {
                        quantity = key_value.as_u64().unwrap_or(quantity);
                    }
                    "equipmentType" => {
                        name = key_value.as_str().unwrap_or(&name).to_string();
                        is_equipment_type = true;
                    }
                    "displayName" => {
                        display_name = key_value.as_str().unwrap_or(&display_name).to_string();
                    }
                    "value" => {
                        value = key_value.as_u64().unwrap_or(value);
                        is_money = true;
                    }
                    _ => {
                        println!("key not accounted for: {:?} -> {:?}", key, value);
                    }
                }
            }
        }

        return StartingItem {
            name,
            quantity,
            value,
            contains_value,
            display_name,
            is_pouch,
            is_special,
            is_equipment_type,
            is_money,
        };
    }
}

#[derive(Debug)]
pub struct StartingEquipment {
    items: Vec<StartingItem>,
    choose_between: Vec<Vec<StartingItem>>,
}

impl StartingEquipment {
    pub fn new(object: Option<&Value>) -> StartingEquipment {
        let parsed_arr = serde_as_array_mapping(object, serde_as_object_from_option, Map::new());

        let mut items = vec![];
        let mut choose_between = vec![];
        for category in parsed_arr {
            if category.get("_").is_some() {
                for item in serde_as_array(category.get("_")) {
                    if item.as_array().is_some() {
                        items.extend(item.as_array().unwrap().iter().map(|i| StartingItem::new(i.to_owned())));
                        continue;
                    }
                    items.push(StartingItem::new(item));
                }
            }

            let selections = vec!["a", "b", "c"];
            let size = if category.get("c").is_some() { 3 } else { 2 };
            for i in 0..size {
                let mut option = vec![];
                for item in serde_as_array(category.get(selections[i])) {
                    option.push(StartingItem::new(item));
                }
                choose_between.push(option);
            }
        }

        return StartingEquipment { items, choose_between };
    }
}

#[derive(Debug)]
pub struct ClassPrerequisite {
    name: String,
    level: u64,
    visible: bool,
}

impl ClassPrerequisite {
    pub fn new(object: Option<&Value>) -> ClassPrerequisite {
        let parsed_object = serde_as_object_from_option(object, Map::new());
        let class_object = serde_as_object_from_option(parsed_object.get("class"), Map::new());

        return ClassPrerequisite {
            name: class_object.get("name").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            level: parsed_object.get("level").unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
            visible: class_object.get("visible").unwrap_or(&to_value(false).unwrap()).as_bool().unwrap_or(false),
        };
    }
}

#[derive(Debug)]
pub struct Prerequisite {
    // TODO: add support for choice from, just level, ability, summaries?, feats, other
    campaign_requirement: Vec<String>,
    requires_campaign: bool,
    class_requirement: Vec<ClassPrerequisite>,
    requires_class: bool,
}

impl Prerequisite {
    pub fn new(list: Option<&Value>) -> Prerequisite {
        let mut campaign_requirement = vec![];
        let mut requires_campaign = false;
        let mut class_requirement = vec![];
        let mut requires_class = false;

        if list.is_some() && list.unwrap().as_array().is_some() {
            for object in list.unwrap().as_array().unwrap() {
                let parsed_object = serde_as_object(object, Map::new());
                if parsed_object.get("campaign").is_some() {
                    campaign_requirement.push(object.get("campaign").unwrap().as_str().unwrap_or("N/A").to_string());
                    requires_campaign = true;
                }
                if parsed_object.get("level").is_some() {
                    class_requirement.push(ClassPrerequisite::new(object.get("level")));
                    requires_class = true;
                }
            }
        }

        return Prerequisite {
            campaign_requirement,
            requires_campaign,
            class_requirement,
            requires_class,
        };
    }
}

#[derive(Debug)]
pub struct SkillToolLanguageChoice {
    language_amount: u64,
    tool_amount: u64, // global_count: u64
}

impl SkillToolLanguageChoice {
    pub fn new(object: &Value) -> SkillToolLanguageChoice {
        let mut language_amount = 0;
        let mut tool_amount = 0;
        // let mut global_count = 0;

        if object.as_object().is_some() {
            let parsed_object = object.as_object().unwrap();
            language_amount = parsed_object.get("anyLanguage").unwrap_or(&to_value(language_amount).unwrap()).as_u64().unwrap_or(language_amount);
            tool_amount = parsed_object.get("anyTool").unwrap_or(&to_value(tool_amount).unwrap()).as_u64().unwrap_or(tool_amount);
        }

        return SkillToolLanguageChoice {
            language_amount,
            tool_amount,
            // global_count
        };
    }
}

#[derive(Debug)]
pub struct SkillToolLanguageProficiencies {
    options: Vec<SkillToolLanguageChoice>,
}

impl SkillToolLanguageProficiencies {
    pub fn new(list: Option<&Value>) -> SkillToolLanguageProficiencies {
        let mut options: Vec<SkillToolLanguageChoice> = vec![];

        if list.is_some() && list.unwrap().as_array().is_some() {
            for option in list.unwrap().as_array().unwrap() {
                options.push(SkillToolLanguageChoice::new(option));
            }
        }

        return SkillToolLanguageProficiencies { options };
    }
}

/// '*' as a key in progression is represented as 0 in the HashMap
#[derive(Debug)]
pub struct OptionalFeatureProgression {
    name: String,
    freature_type: Vec<String>,
    progression: HashMap<u64, u64>,
}

impl OptionalFeatureProgression {
    fn form_progression_map() -> HashMap<u64, u64> {
        let mut progression_map = HashMap::new();
        for i in 0..21 {
            progression_map.insert(i, 0);
        }
        return progression_map;
    }

    pub fn new(value: Option<&Value>) -> OptionalFeatureProgression {
        let object = serde_as_object_from_option(value, Map::new());

        let mut progression = OptionalFeatureProgression::form_progression_map();
        if object.contains_key("progression") {
            let progression_value = object.get("progression").unwrap();
            if progression_value.is_array() {
                for (index, item) in serde_as_array_mapping(Some(progression_value), serde_as_u64, 0).iter().enumerate() {
                    progression.insert(u64::try_from(index + 1).unwrap_or(0), item.to_owned());
                }
            } else if progression_value.is_object() {
                for (key, value) in progression_value.as_object().unwrap() {
                    progression.insert(key.parse::<u64>().unwrap_or(0), value.as_u64().unwrap_or(0));
                }
            } else {
                println!("unexpected value type for optionalFeatureProgression progression key");
            }
        }

        return OptionalFeatureProgression {
            name: serde_as_string(object.get("name"), "N/A".to_string()),
            freature_type: serde_as_array_mapping(object.get("featureType"), serde_as_string, "N/A".to_string()),
            progression: HashMap::new(),
        };
    }
}

/**
 * Only found in one feat (Resilient)
 * so not a lot to go off of
 */
#[derive(Debug)]
pub struct SavingThrowProficiencies {
    options: Vec<String>,
    amount: u64,
}

impl SavingThrowProficiencies {
    pub fn new(object: Option<&Value>) -> SavingThrowProficiencies {
        let mut options = vec![];
        let mut amount = 0;

        if object.is_some() && object.unwrap().as_object().is_some() {
            let parsed_object = object.unwrap().as_object().unwrap();
            options = parsed_object.get("from").unwrap().as_array().unwrap().iter().map(|f| f.as_str().unwrap_or("N/A").to_string()).collect();
            amount = parsed_object.get("count").unwrap_or(&to_value(amount).unwrap()).as_u64().unwrap_or(amount);
        }

        return SavingThrowProficiencies { options, amount };
    }
}

/**
 * Only found in one feat (Skill Expert)
 * so not a lot to go off of
 */
#[derive(Debug)]
pub struct Expertise {
    skill: String,
    can_choose: bool,
    amount: u64,
}

impl Expertise {
    pub fn new(object: Option<&Value>) -> Expertise {
        let mut skill = "N/A".to_string();
        let mut can_choose = false;
        let mut amount = 0;

        if object.is_some() && object.unwrap().as_object().is_some() {
            let parsed_object = object.unwrap().as_object().unwrap();
            skill = parsed_object.keys().into_iter().collect::<Vec<_>>()[0].to_owned();
            can_choose = skill == "anyProficientSkill";
            amount = parsed_object.get(&skill).unwrap_or(&to_value(amount).unwrap()).as_u64().unwrap_or(amount);
        }

        return Expertise { skill, can_choose, amount };
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    name: String,
    entry_type: String,
    content: Vec<String>,
}

impl Entry {
    pub fn new(object: &Value) -> Entry {
        let parsed_object = serde_as_object(object, Map::new());

        return Entry {
            name: parsed_object.get("name").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            entry_type: parsed_object.get("type").unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
            content: parsed_object.get("entries").unwrap_or(&to_value::<Vec<Value>>(vec![]).unwrap()).as_array().unwrap_or(&vec![]).iter().map(|i| i.as_str().unwrap_or("N/A").to_string()).collect(),
        };
    }

    pub fn default() -> Entry {
        return Entry {
            name: "N/A".to_string(),
            entry_type: "N/A".to_string(),
            content: vec![],
        };
    }
}

#[derive(Debug)]
pub struct Details {
    pub age: u64,
    pub eyes: String,
    pub hair: String,
    pub skin: String,
    pub weight: u64,
    pub height: String,
	pub appearance: String,
    pub personality: String,
    pub ideal: String,
    pub bond: String,
    pub flaw: String,
    pub backstory: String,
    pub physical: String,
    pub alignment: String,
	pub allies_and_organizations: String,
	pub additional_features_and_traits: String,
	pub treasure: String,
}

#[derive(Debug)]
pub struct CharacterItem {
    pub name: String,
    pub quantity: u64,
    pub description: String,
    pub equipped: bool,
    pub item: Item,
}

#[derive(Debug)]
pub struct Currency {
    pub platinum_piece: u64,
    pub electrum_piece: u64,
    pub gold_piece: u64,
    pub silver_piece: u64,
    pub copper_piece: u64,
}

#[derive(Debug)]
pub struct Die {
    pub number: u64,
    pub faces: u64,
}

impl Die {
    pub fn new_from_hit_die(value: Option<&Value>) -> Die {
        let object = serde_as_object_from_option(value, Map::new());

        return Die {
            number: serde_as_u64(object.get("number"), 0),
            faces: serde_as_u64(object.get("faces"), 0),
        };
    }
}

#[derive(Debug)]
pub struct ClassWeaponProficiencies {
    pub weapons: Vec<String>,
    pub optional_weapons: Vec<String>,
}

impl ClassWeaponProficiencies {
    pub fn new(value: Option<&Value>) -> ClassWeaponProficiencies {
        let array = serde_as_array(value);

        let mut weapons: Vec<String> = vec![];
        let mut optional_weapons: Vec<String> = vec![];

        for item in array {
            if item.is_string() {
                weapons.push(item.as_str().unwrap().to_string());
            } else if item.is_object() {
                let object = item.as_object().unwrap();
                if object.contains_key("optional") {
                    optional_weapons.push(serde_as_string(object.get("proficiency"), "N/A".to_string()));
                }
            } else {
                println!("item in weapon proficiencies was something other than a string or an object");
            }
        }

        return ClassWeaponProficiencies { weapons, optional_weapons };
    }
}

#[derive(Debug)]
pub struct Proficiencies {
    pub armor: ArmorProficiencies, // TODO: check to see if this is valid for the data
    pub weapons: ClassWeaponProficiencies,
    pub tools: Vec<String>,
    pub tools_proficiencies: ToolProficiencies, // TODO: check to see if this is valid for the data
    pub skills: SkillProficiencies,             // TODO: check to see if this is valid for the data
}

impl Proficiencies {
    pub fn new(value: Option<&Value>) -> Proficiencies {
        let object = serde_as_object_from_option(value, Map::new());

        return Proficiencies {
            armor: ArmorProficiencies::new(object.get("armor")),
            weapons: ClassWeaponProficiencies::new(object.get("weapons")),
            tools: serde_as_array_mapping(object.get("tools"), serde_as_string, "N/A".to_string()),
            tools_proficiencies: ToolProficiencies::new(object.get("toolsProficiencies")),
            skills: SkillProficiencies::new(object.get("skills")),
        };
    }
}

#[derive(Debug)]
pub struct ClassStartingEquipment {
    pub additional_from_background: bool,
    pub default: Vec<String>,
    pub gold_alternative: String,
    pub default_data: StartingEquipment,
}

impl ClassStartingEquipment {
    pub fn new(value: Option<&Value>) -> ClassStartingEquipment {
        let object = serde_as_object_from_option(value, Map::new());

        return ClassStartingEquipment {
            additional_from_background: serde_as_bool(object.get("additionalFromBackground"), false),
            default: serde_as_array_mapping(object.get("default"), serde_as_string, "N/A".to_string()),
            gold_alternative: serde_as_string(object.get("goldAlternative"), "N/A".to_string()),
            default_data: StartingEquipment::new(object.get("defaultData")),
        };
    }
}

#[derive(Debug)]
pub struct MulticlassingRequirements {
    pub con: u64,
    pub dex: u64,
    pub str: u64,
    pub int: u64,
    pub wis: u64,
    pub cha: u64,
}

impl MulticlassingRequirements {
    pub fn new(value: Option<&Value>) -> MulticlassingRequirements {
        let object = serde_as_object_from_option(value, Map::new());

        return MulticlassingRequirements {
            con: serde_as_u64(object.get("con"), 0),
            dex: serde_as_u64(object.get("dex"), 0),
            str: serde_as_u64(object.get("str"), 0),
            int: serde_as_u64(object.get("int"), 0),
            wis: serde_as_u64(object.get("wis"), 0),
            cha: serde_as_u64(object.get("cha"), 0),
        };
    }
}

#[derive(Debug)]
pub struct Multiclassing {
    pub requirements: MulticlassingRequirements,
    pub proficiencies_gained: Proficiencies,
}

impl Multiclassing {
    pub fn new(value: Option<&Value>) -> Multiclassing {
        let object = serde_as_object_from_option(value, Map::new());

        return Multiclassing {
            requirements: MulticlassingRequirements::new(object.get("requirements")),
            proficiencies_gained: Proficiencies::new(object.get("proficienciesGained")),
        };
    }
}

#[derive(Debug)]
pub struct ClassTableGroup {
    pub title: String,
    pub column_labels: Vec<String>,
    pub rows: Vec<Vec<u64>>,
    pub rows_spell_progress: Vec<Vec<u64>>,
}

impl ClassTableGroup {
    pub fn new(value: &Value) -> ClassTableGroup {
        let object = serde_as_object(value, Map::new());

        return ClassTableGroup {
            title: serde_as_string(object.get("title"), "N/A".to_string()),
            column_labels: serde_as_array_mapping(object.get("colLabels"), serde_as_string, "N/A".to_string()),
            rows: serde_as_array(object.get("rows")).iter().map(|i| serde_as_array_mapping(Some(i), serde_as_u64, 0)).collect(),
            rows_spell_progress: serde_as_array(object.get("rowsSpellProgression")).iter().map(|i| serde_as_array_mapping(Some(i), serde_as_u64, 0)).collect(),
        };
    }
}
