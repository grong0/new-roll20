use serde_json::{Map, Value};

use super::common::{form_key, Entry, Source};

use crate::serde_utils::{serde_as_array, serde_as_bool, serde_as_object, serde_as_string};

#[derive(Debug)]
pub struct Skill {
	pub name: String,
	pub source: Source,
	pub key: String,
	pub srd: bool,
	pub basic_rules: bool,
	pub entries: Vec<Entry>,
}

impl Skill {
	pub fn new(value: Value) -> Skill {
		let object = serde_as_object(&value, Map::new());

		let name = serde_as_string(object.get("name"), "N/A".to_string());
		let source = Source::new(object.get("source"), object.get("page"));

		return Skill {
			key: form_key(&name, &source.name),
			name,
			source,
			srd: serde_as_bool(object.get("srd"), false),
			basic_rules: serde_as_bool(object.get("basicRules"), false),
			entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
		};
	}
}
