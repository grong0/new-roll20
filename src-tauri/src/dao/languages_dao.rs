use serde_json::{Map, Value};

use super::common::{form_key, Entry, Source};

use crate::serde_utils::{
	serde_as_array, serde_as_array_mapping, serde_as_bool, serde_as_object, serde_as_object_from_option, serde_as_string,
};

#[derive(Debug)]
pub struct Language {
	pub name: String,
	pub source: Source,
	pub key: String,
	pub typical_speakers: Vec<String>,
	pub language_type: String,
	pub script: String,
	pub srd: bool,
	pub basic_rules: bool,
	pub entries: Vec<Entry>,
	pub has_fluff_images: bool,
	pub fonts: Vec<String>,
	pub additional_sources: Vec<Source>,
	pub dialects: Vec<String>,
	pub other_sources: Vec<Source>,
}

impl Language {
	pub fn new(value: Value) -> Language {
		let object = serde_as_object(&value, Map::new());

		let name = serde_as_string(object.get("name"), "N/A".to_string());
		let source = Source::new(object.get("source"), object.get("page"));

		return Language {
			key: form_key(&name, &source.name),
			name,
			source,
			typical_speakers: serde_as_array_mapping(object.get("typicalSpeakers"), serde_as_string, "N/A".to_string()),
			language_type: serde_as_string(object.get("type"), "N/A".to_string()),
			script: serde_as_string(object.get("script"), "N/A".to_string()),
			srd: serde_as_bool(object.get("srd"), false),
			basic_rules: serde_as_bool(object.get("basicRules"), false),
			entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
			has_fluff_images: serde_as_bool(object.get("hasFluffImages"), false),
			fonts: serde_as_array_mapping(object.get("fonts"), serde_as_string, "N/A".to_string()),
			additional_sources: serde_as_array_mapping(object.get("additionalSources"), serde_as_object_from_option, Map::new())
				.iter()
				.map(|i| Source::new(i.get("source"), i.get("page")))
				.collect(),
			dialects: serde_as_array_mapping(object.get("dialects"), serde_as_string, "N/A".to_string()),
			other_sources: serde_as_array_mapping(object.get("otherSources"), serde_as_object_from_option, Map::new())
				.iter()
				.map(|i| Source::new(i.get("source"), i.get("page")))
				.collect(),
		};
	}
}
