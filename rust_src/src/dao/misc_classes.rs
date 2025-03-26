use serde_json::{to_value, Value};

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
	pub fn new(object: Option<&Value>) {
		let base_height: Option<Value> = object.get("baseHeight");
		let height_mod: Option<Value> = object.get("heightMod");
		let base_weight: Option<Value> = object.get("baseWeight");
		let weight_mod: Option<Value> = object.get("weightMod");

		return HeightAndWeight {
			base_height: base_height.unwrap_or(&to_value(0).unwrap()).as_u64().unwrap_or(0),
			height_mod: height_mod.unwrap_or(&to_value("N/A").unwrap()).as_str().unwrap_or("N/A"),
			base_weight: base_weight,
			weight_mod: weight_mod
		}
	}
}

#[derive(Debug)]
pub struct Age {

}

#[derive(Debug)]
pub struct SkillProficiencies {}

#[derive(Debug)]
pub struct LanguageProficiencies {}

#[derive(Debug)]
pub struct ToolProficiencies {}

#[derive(Debug)]
pub struct WeaponProficiencies {}

#[derive(Debug)]
pub struct ArmorProficiencies {}

#[derive(Debug)]
pub struct Resist {}

#[derive(Debug)]
pub struct AdditionalSpells {}
