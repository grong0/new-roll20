use std::any::type_name;

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
	weight_mod: String
}

impl HeightAndWeight {
	pub fn new(base_height: Option<&Value>, height_mod: Option<&Value>, base_weight: Option<&Value>, weight_mod: Option<&Value>) -> HeightAndWeight {
		return HeightAndWeight {
			base_height: base_height.unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
			height_mod: height_mod.unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string(),
			base_weight: base_weight.unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
			weight_mod: weight_mod.unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string()
		}
	}
}

#[derive(Debug)]
pub struct Age {
	mature: u64,
	max: u64
}

impl Age {
	pub fn new(mature: Option<&Value>, max: Option<&Value>) -> Age {
		return Age {
			mature: mature.unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
			max: max.unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0)
		}
	}
}

#[derive(Debug)]
pub struct SkillProficiencies {
	skills: Vec<String>,
	any: u64,
	choose_skills: Vec<String>,
	choose_count: u64
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
				choose_skills = choose.get("from").unwrap().as_array().unwrap().iter().map(|skl| skl.to_string()).collect();
				choose_count = choose.get("count").unwrap_or(&to_value(1).unwrap()).as_u64().unwrap_or(1);
			} else {
				skills.push(key.to_string());
			}
		}

		return SkillProficiencies {
			skills,
			any,
			choose_skills,
			choose_count
		}
	}
}

#[derive(Debug)]
pub struct LanguageProficiencies {
	languages: Vec<String>,
	any_standard: u64,
	choose_languages: Vec<String>,
	choose_count: u64
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
				choose_languages = choose.get("from").unwrap().as_array().unwrap().iter().map(|lan| lan.to_string()).collect();
				choose_count = choose.get("count").unwrap_or(&to_value(1).unwrap()).as_u64().unwrap_or(1);
			} else {
				languages.push(key.to_string());
			}
		}

		return LanguageProficiencies {
			languages,
			any_standard,
			choose_languages,
			choose_count
		}
	}
}

#[derive(Debug)]
pub struct ToolProficiencies {
	tools: Vec<String>,
	choose_any_amount: u64
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
			choose_any_amount
		}
	}
}

#[derive(Debug)]
pub struct WeaponProficiencies {
	weapons: Vec<String>,
	choose_filter: String, // Only from one race: Hobgoblin|VGM
	choose_amount: u64
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
			choose_amount
		}
	}
}

#[derive(Debug)]
pub struct ArmorProficiencies {
	armor: Vec<String>
}

impl ArmorProficiencies {
	pub fn new(object: Option<&Value>) -> ArmorProficiencies {
		let parsed_object = object.unwrap().as_object().unwrap(); // TODO: add defaults here

		return ArmorProficiencies {
			armor: parsed_object.keys().into_iter().map(|arm| arm.to_string()).collect()
		}
	}
}

#[derive(Debug)]
pub struct Resist {
	resistances: Vec<String>,
	choose_from: Vec<String>
}

impl Resist {
	pub fn new(list: Option<&Value>) -> Resist {
		let mut resistances: Vec<String> = vec![];
		let mut choose_from: Vec<String> = vec![];

		for item in list.unwrap().as_array().unwrap() { // TODO: add defaults here
			if item.is_object() {
				choose_from = item.as_object().unwrap().get("from").unwrap().as_array().unwrap().iter().map(|res| res.to_string()).collect();
			} else {
				resistances.push(item.as_str().unwrap_or("").to_string());
			}
		}

		return Resist {
			resistances,
			choose_from
		}
	}
}

#[derive(Debug)]
enum Reset {
	NEVER,
	REST,
	DAILY
}

#[derive(Debug)]
pub struct AdditionalSpell {
	name: String,
	ability: String,
	reset_when: Reset,
	aquired_at: u64
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
	choose: Vec<AdditionalSpell>
}

impl AdditionalSpells {
	pub fn run(list: Option<&Value>) -> AdditionalSpells {
		return AdditionalSpells {
			spells: vec![],
			choose: vec![]
		}
	}
}

#[derive(Debug)]
pub struct Time {
	quantity: u64,
	unit: String
}

impl Time {
	pub fn new(quantity: Option<&Value>, unit: Option<&Value>) -> Time {
		return Time {
			quantity: quantity.unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
			unit: unit.unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A").to_string()
		}
	}
}

#[derive(Debug)]
pub struct Range {
	typer: String,
	form: String,
	amount: u64
}

// impl Range {
// 	pub fn new()
// }
