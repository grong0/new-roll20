use serde_json::{Map, Value};

use super::common::{form_key, serde_as_array_mapping, serde_as_bool, serde_as_object_from_option, serde_as_string, Source};

#[derive(Debug)]
pub struct Skill {
	name: String,
	source: Source,
	key: String,
	srd: bool,
	basic_rules: bool,
	entries: Vec<String>
}

impl Skill {
	pub fn new(value: Option<&Value>) -> Skill {
		let object = serde_as_object_from_option(value, Map::new());

		let name = serde_as_string(object.get("name"), "N/A".to_string());
		let source = Source::new(object.get("source"), object.get("page"));

		return Skill {
			key: form_key(&name, &source.name),
			name,
			source,
			srd: serde_as_bool(object.get("srd"), false),
			basic_rules: serde_as_bool(object.get("basicRules"), false),
			entries: serde_as_array_mapping(object.get("entries"), serde_as_string, "N/A".to_string())
		}
	}
}
