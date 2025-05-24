use std::{any::type_name, iter::zip};

use serde_json::{to_value, Map, Value};

#[derive(Debug)]
pub struct Source {
    pub name: String,
    pub page: u64,
}

impl Source {
    pub fn new(source: Option<&Value>, page: Option<&Value>) -> Source {
        return Source {
            name: source
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
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
    pub fn new(
        base_height: Option<&Value>,
        height_mod: Option<&Value>,
        base_weight: Option<&Value>,
        weight_mod: Option<&Value>,
    ) -> HeightAndWeight {
        return HeightAndWeight {
            base_height: base_height
                .unwrap_or(&to_value(0).unwrap())
                .as_u64()
                .unwrap_or(0),
            height_mod: height_mod
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            base_weight: base_weight
                .unwrap_or(&to_value(0).unwrap())
                .as_u64()
                .unwrap_or(0),
            weight_mod: weight_mod
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
        };
    }
}

#[derive(Debug)]
pub struct Age {
    mature: u64,
    max: u64,
}

impl Age {
    pub fn new(mature: Option<&Value>, max: Option<&Value>) -> Age {
        return Age {
            mature: mature
                .unwrap_or(&to_value(0).unwrap())
                .as_u64()
                .unwrap_or(0),
            max: max.unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
        };
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

        let parsed_object = object.unwrap().as_object().unwrap(); // TODO: add defaults here

        for (key, value) in parsed_object {
            if key == "any" {
                any = value.as_u64().unwrap();
            } else if key == "choose" {
                let choose = value.as_object().unwrap();
                choose_skills = choose
                    .get("from")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|skl| skl.to_string())
                    .collect(); // TODO: add defaults here
                choose_count = choose
                    .get("count")
                    .unwrap_or(&to_value(1).unwrap())
                    .as_u64()
                    .unwrap_or(1);
            } else {
                skills.push(key.to_string());
            }
        }

        return SkillProficiencies {
            skills,
            any,
            choose_skills,
            choose_count,
        };
    }
}

#[derive(Debug)]
pub struct LanguageProficiencies {
    languages: Vec<String>,
    any_standard: u64,
    choose_languages: Vec<String>,
    choose_count: u64,
}

// TODO: add support for getting the 'other' langage from entries
impl LanguageProficiencies {
    pub fn new(object: Option<&Value>) -> LanguageProficiencies {
        let mut languages: Vec<String> = vec![];
        let mut any_standard = 0;
        let mut choose_languages: Vec<String> = vec![];
        let mut choose_count = 0;

        let parsed_object = object.unwrap().as_object().unwrap(); // TODO: add defaults here

        for (key, value) in parsed_object {
            if key == "anyStandard" {
                any_standard = value.as_u64().unwrap();
            } else if key == "choose" {
                let choose = value.as_object().unwrap();
                choose_languages = choose
                    .get("from")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|lan| lan.to_string())
                    .collect();
                choose_count = choose
                    .get("count")
                    .unwrap_or(&to_value(1).unwrap())
                    .as_u64()
                    .unwrap_or(1);
            } else {
                languages.push(key.to_string());
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

        let parsed_object = object.unwrap().as_object().unwrap(); // TODO: add defaults here

        for (key, value) in parsed_object {
            if key == "any" {
                choose_any_amount = value.as_u64().unwrap()
            } else {
                tools.push(key.to_string());
            }
        }

        return ToolProficiencies {
            tools,
            choose_any_amount,
        };
    }
}

#[derive(Debug)]
pub struct WeaponProficiencies {
    weapons: Vec<String>,
    choose_filter: String, // Only from one race: Hobgoblin|VGM
    choose_amount: u64,
}

impl WeaponProficiencies {
    pub fn new(object: Option<&Value>) -> WeaponProficiencies {
        let mut weapons: Vec<String> = vec![];
        let mut choose_filter = "".to_string();
        let mut choose_amount = 0;

        let parsed_object = object.unwrap().as_object().unwrap(); // TODO: add defaults here

        for (key, value) in parsed_object {
            if key == "choose" {
                let choose = value.as_object().unwrap();
                choose_filter = choose.get("filter").unwrap().as_str().unwrap().to_string();
                choose_amount = choose.get("count").unwrap().as_u64().unwrap()
            } else {
                weapons.push(key.to_string());
            }
        }

        return WeaponProficiencies {
            weapons,
            choose_filter,
            choose_amount,
        };
    }
}

#[derive(Debug)]
pub struct ArmorProficiencies {
    armor: Vec<String>,
}

impl ArmorProficiencies {
    pub fn new(object: Option<&Value>) -> ArmorProficiencies {
        let parsed_object = object.unwrap().as_object().unwrap(); // TODO: add defaults here

        return ArmorProficiencies {
            armor: parsed_object
                .keys()
                .into_iter()
                .map(|arm| arm.to_string())
                .collect(),
        };
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

        for item in list.unwrap().as_array().unwrap() {
            // TODO: add defaults here
            if item.is_object() {
                choose_from = item
                    .as_object()
                    .unwrap()
                    .get("from")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|res| res.to_string())
                    .collect();
            } else {
                resistances.push(item.as_str().unwrap_or("").to_string());
            }
        }

        return Resist {
            resistances,
            choose_from,
        };
    }
}

#[derive(Debug)]
enum Reset {
    NEVER,
    REST,
    DAILY,
}

#[derive(Debug)]
pub struct AdditionalSpell {
    name: String,
    ability: String,
    reset_when: Reset,
    aquired_at: u64,
}

/**
 * There are 4 keys:
 * 1. known | these spells have no level requirement, but are tied with a resource
 * 2. innate | these are like known, but have a level requirement
 * 3. expanded | expands your spellbook?
 * 4. ability | the ability the other spells use
 */
#[derive(Debug)]
pub struct AdditionalSpells {
    spells: Vec<AdditionalSpell>,
    choose: Vec<AdditionalSpell>,
}

impl AdditionalSpells {
    pub fn new(list: Option<&Value>) -> AdditionalSpells {
        return AdditionalSpells {
            spells: vec![],
            choose: vec![],
        };
    }
}

#[derive(Debug)]
pub struct Time {
    quantity: u64,
    unit: String,
}

impl Time {
    pub fn new(quantity: Option<&Value>, unit: Option<&Value>) -> Time {
        return Time {
            quantity: quantity
                .unwrap_or(&to_value(0).unwrap())
                .as_u64()
                .unwrap_or(0),
            unit: unit
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
        };
    }
}

#[derive(Debug)]
pub struct Range {
    typer: String,
    form: String,
    amount: u64,
}

impl Range {
    pub fn new(typer: Option<&Value>, distance: Option<&Value>) -> Range {
        let parsed_distance = distance.unwrap().as_object().unwrap(); // TODO: add defaults here
        return Range {
            typer: typer
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            form: parsed_distance
                .get("type")
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            amount: parsed_distance
                .get("amount")
                .unwrap_or(&to_value(0).unwrap())
                .as_u64()
                .unwrap_or(0),
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
    pub fn new(
        v: Option<&Value>,
        s: Option<&Value>,
        m: Option<&Value>,
        r: Option<&Value>,
    ) -> Components {
        return Components {
            v: v.unwrap_or(&to_value(false).unwrap())
                .as_bool()
                .unwrap_or(false),
            s: s.unwrap_or(&to_value(false).unwrap())
                .as_bool()
                .unwrap_or(false),
            m: m.unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            r: r.unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
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
        let parsed_object = object.unwrap().as_object().unwrap(); // TODO: add defaults here
        let mut ends = vec![];

        for end in parsed_object.get("ends").unwrap().as_array().unwrap() {
            // TODO: add defaults here
            ends.push(end.as_str().unwrap_or("N/A").to_string());
        }

        let mut unit: String = "N/A".to_string();
        let mut amount: u64 = 0;

        match parsed_object.get("duration") {
            Some(duration) => {
                unit = duration
                    .get("type")
                    .unwrap_or(&to_value("N/A").unwrap())
                    .as_str()
                    .unwrap_or("N/A")
                    .to_string();
                amount = duration
                    .get("amount")
                    .unwrap_or(&to_value(0).unwrap())
                    .as_u64()
                    .unwrap_or(0);
            }
            None => {}
        }

        return Duration {
            typed: parsed_object
                .get("type")
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            unit,
            amount,
            concentration: parsed_object
                .get("concentration")
                .unwrap_or(&to_value(false).unwrap())
                .as_bool()
                .unwrap_or(false),
            ends,
        };
    }
}

#[derive(Debug)]
struct LevelDie {
    level: String,
    die: String,
}

#[derive(Debug)]
struct ScalingLevelDice {
    label: String,
    scaling: Vec<LevelDie>,
}

impl ScalingLevelDice {
    pub fn new(object: Option<&Value>) -> ScalingLevelDice {
        let parsed_object = object.unwrap().as_object().unwrap(); // TODO: add defaults here

        let mut scaling = vec![];
        let scaling_object = parsed_object.get("scaling").unwrap().as_object().unwrap(); // TODO: add defaults here
        for level in scaling_object.keys() {
            scaling.push(LevelDie {
                level: level.to_string(),
                die: scaling_object
                    .get(level)
                    .unwrap_or(&to_value("N/A").unwrap())
                    .as_str()
                    .unwrap_or("N/A")
                    .to_string(),
            });
        }

        return ScalingLevelDice {
            label: parsed_object
                .get("label")
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            scaling,
        };
    }
}

#[derive(Debug)]
struct PackItem {
    name: String,
    quantity: u64,
    special: bool,
}

impl PackItem {
    pub fn new(item: Option<&Value>) -> PackItem {
        match item.unwrap().as_object() {
            // TODO: add defaults here
            Some(object) => {
                let name: String;
                match object.get("special") {
                    Some(special) => {
                        name = special.as_str().unwrap_or("NOT_STRING_SPECIAL").to_string();
                    }
                    None => {
                        name = object
                            .get("item")
                            .unwrap_or(&to_value("N/A").unwrap())
                            .as_str()
                            .unwrap_or("N/A")
                            .to_string();
                    }
                }
                return PackItem {
                    name,
                    quantity: object
                        .get("quantity")
                        .unwrap_or(&to_value(1).unwrap())
                        .as_u64()
                        .unwrap_or(1),
                    special: false,
                };
            }
            None => {
                return PackItem {
                    name: item
                        .unwrap_or(&to_value("N/A").unwrap())
                        .as_str()
                        .unwrap_or("N/A")
                        .to_string(),
                    quantity: 1,
                    special: false,
                }
            }
        }
    }
}

#[derive(Debug)]
enum ItemType {
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
        match item_type
            .unwrap_or(&to_value("N/A").unwrap())
            .as_str()
            .unwrap_or("N/A")
        {
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
struct ValidContainerSlotItem {
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
struct ContainerSlot {
    weight_limit: u64,
    valid_items: Vec<ValidContainerSlotItem>,
}

impl ContainerSlot {
    pub fn new(weight_limit: u64, items_object: Map<String, Value>) -> ContainerSlot {
        let mut valid_items: Vec<ValidContainerSlotItem> = vec![];
        for (key, value) in items_object {
            valid_items.push(ValidContainerSlotItem::new(key, value));
        }

        return ContainerSlot {
            weight_limit,
            valid_items,
        };
    }
}

#[derive(Debug)]
struct ContainerCapacity {
    slots: Vec<ContainerSlot>,
    weightless: bool,
}

impl ContainerCapacity {
    pub fn new(object: Option<&Value>) -> ContainerCapacity {
        let parsed_object = object.unwrap().as_object().unwrap(); // TODO: add defaults here

        // Parses weight and item specifications
        let mut parsed_weights: Vec<u64> = vec![];
        match parsed_object.get("weight") {
            Some(weights) => {
                for weight in weights.as_array().unwrap() {
                    // TODO: add defaults here
                    parsed_weights.push(weight.as_u64().unwrap_or(0));
                }
            }
            None => {}
        }
        let mut parsed_items: Vec<Map<String, Value>> = vec![];
        match parsed_object.get("item") {
            Some(items) => {
                for item in items.as_array().unwrap() {
                    // TODO: add defaults here
                    parsed_items.push(item.as_object().unwrap().to_owned()); // TODO: add defaults here
                }
            }
            None => {}
        }

        // Extends the smallest list to be the same size as the other
        let parsed_weights_len = parsed_weights.len();
        let parsed_items_len = parsed_items.len();
        if parsed_weights_len < parsed_items_len {
            let extension: Vec<u64> = (0..parsed_items_len - parsed_weights_len)
                .map(|_| 0)
                .collect();
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
            weightless: parsed_object
                .get("weightless")
                .unwrap_or(&to_value(false).unwrap())
                .as_bool()
                .unwrap_or(false),
        };
    }
}

#[derive(Debug)]
struct StartingItem {
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
    pub fn new(item: &Value) -> StartingItem {
        let mut name = "N/A".to_string();
        let mut quantity = 1;
        let mut value = 0;
        let mut contains_value = 0;
        let mut display_name = "".to_string();
        let mut is_pouch = false;
        let mut is_special = false;
        let mut is_equipment_type = false;
        let mut is_money = false;

        match item.unwrap_or(&to_value("N/A").unwrap()).as_object() {
            Some(object) => {
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
            None => {}
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
struct StartingEquipment {
    items: Vec<StartingItem>,
    choose_between: Vec<Vec<StartingItem>>,
}

impl StartingEquipment {
    pub fn new(object: Option<&Value>) -> StartingEquipment {
        let parsed_arr = object
            .unwrap()
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f.as_object().unwrap()); // TODO: add defaults here

        let mut items = vec![];
        let mut choose_between = vec![];
        for category in parsed_arr {
            if category.get("_").is_some() {
                for item in category.get("_").unwrap().as_array().unwrap() {
                    // TODO: add defaults here
                    if item.as_array().is_some() {
                        items.extend(
                            item.as_array()
                                .unwrap()
                                .iter()
                                .map(|f| StartingItem::new(f)),
                        );
                        continue;
                    }
                    items.push(StartingItem::new(item));
                }
            }

            let selections = vec!["a", "b", "c"];
            let size = if category.get("c").is_some() { 3 } else { 2 };
            for i in 0..size {
                let mut option = vec![];
                for item in category.get(selections[i]).unwrap().as_array().unwrap() {
                    // TODO: add defaults here
                    option.push(StartingItem::new(item));
                }
                choose_between.push(option);
            }
        }

        return StartingEquipment {
            items,
            choose_between,
        };
    }
}

#[derive(Debug)]
struct ClassPrerequisite {
    name: String,
    level: u64,
    visible: bool,
}

impl ClassPrerequisite {
    pub fn new(object: Option<&Value>) -> ClassPrerequisite {
        let parsed_object = object.unwrap().as_object().unwrap(); // TODO: add defaults here
        let class_object = parsed_object.get("class").unwrap().as_object().unwrap(); // TODO: add defaults here

        return ClassPrerequisite {
            name: class_object
                .get("name")
                .unwrap_or(&to_value("N/A").unwrap())
                .as_str()
                .unwrap_or("N/A")
                .to_string(),
            level: parsed_object
                .get("level")
                .unwrap_or(&to_value(0).unwrap())
                .as_u64()
                .unwrap_or(0),
            visible: class_object
                .get("visible")
                .unwrap_or(&to_value(false).unwrap())
                .as_bool()
                .unwrap_or(false),
        };
    }
}

#[derive(Debug)]
struct Prerequisite { // TODO: add support for choice from, just level, ability, summaries?, feats, other
	campaign_requirement: Vec<String>,
	requires_campaign: bool,
	class_requirement: Vec<ClassPrerequisite>,
	requires_class: bool
}

impl Prerequisite {
	pub fn new(list: Option<&Value>) -> Prerequisite {
		let mut campaign_requirement = vec![];
		let mut requires_campaign = false;
		let mut class_requirement = vec![];
		let mut requires_class = false;

		if list.is_some() && list.unwrap().as_array().is_some() {
			for object in list.unwrap().as_array().unwrap() {
				let parsed_object = object.as_object().unwrap(); // TODO: add defaults here
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
			requires_class
		}
	}
}

#[derive(Debug)]
struct SkillToolLanguageChoice {
	language_amount: u64,
	tool_amount: u64
	// global_count: u64
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
		}
	}
}

#[derive(Debug)]
struct SkillToolLanguageProficiencies {
	options: Vec<SkillToolLanguageChoice>
}

impl SkillToolLanguageProficiencies {
	pub fn new(list: Option<&Value>) -> SkillToolLanguageProficiencies {
		let mut options: Vec<SkillToolLanguageChoice> = vec![];

		if list.is_some() && list.unwrap().as_array().is_some() {
			for option in list.unwrap().as_array().unwrap() {
				options.push(SkillToolLanguageChoice::new(option));
			}
		}

		return SkillToolLanguageProficiencies {
			options
		}
	}
}


