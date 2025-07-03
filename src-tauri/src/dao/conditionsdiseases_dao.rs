use serde_json::{Map, Value};

use super::common::{form_key, Entry, Source};

use crate::serde_utils::{
	serde_as_array, serde_as_array_mapping, serde_as_bool, serde_as_object, serde_as_object_from_option, serde_as_string,
};

#[derive(Debug)]
pub struct Condition {
	pub name: String,
	pub source: Source,
	pub key: String,
	pub srd: bool,
	pub basic_rules: bool,
	pub other_sources: Vec<Source>,
	pub entries: Vec<Entry>,
	pub has_fluff_images: bool,
}

impl Condition {
	pub fn new(value: Value) -> Condition {
		let object = serde_as_object(&value, Map::new());

		let name = serde_as_string(object.get("name"), "N/A".to_string());
		let source = Source::new(object.get("source"), object.get("pageg"));

		return Condition {
			key: form_key(&name, &source.name),
			name,
			source,
			srd: serde_as_bool(object.get("srd"), false),
			basic_rules: serde_as_bool(object.get("basicRules"), false),
			other_sources: serde_as_array_mapping(object.get("otherSources"), serde_as_object_from_option, Map::new())
				.iter()
				.map(|i| Source::new(i.get("source"), i.get("page")))
				.collect(),
			entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
			has_fluff_images: serde_as_bool(object.get("hasFluffImages"), false),
		};
	}
}

#[derive(Debug)]
pub struct Disease {
	pub name: String,
	pub source: Source,
	pub key: String,
	pub entries: Vec<Entry>,
	pub srd: bool,
}

impl Disease {
	pub fn new(value: Value) -> Disease {
		let object = serde_as_object(&value, Map::new());

		let name = serde_as_string(object.get("name"), "N/A".to_string());
		let source = Source::new(object.get("source"), object.get("pageg"));

		return Disease {
			key: form_key(&name, &source.name),
			name,
			source,
			entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
			srd: serde_as_bool(object.get("srd"), false),
		};
	}
}

#[derive(Debug)]
pub struct Status {
	pub name: String,
	pub source: Source,
	pub key: String,
	pub srd: bool,
	pub basic_rules: bool,
	pub entries: Vec<Entry>,
}

impl Status {
	pub fn new(value: Value) -> Status {
		let object = serde_as_object(&value, Map::new());

		let name = serde_as_string(object.get("name"), "N/A".to_string());
		let source = Source::new(object.get("source"), object.get("pageg"));

		return Status {
			key: form_key(&name, &source.name),
			name,
			source,
			srd: serde_as_bool(object.get("srd"), false),
			basic_rules: serde_as_bool(object.get("basicRules"), false),
			entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
		};
	}
}
