use serde_json::{to_value, Map, Value};

use super::common::{form_key, serde_as_array, serde_as_bool, serde_as_object, serde_as_string, Entry, Source, Time};

pub struct Action {
	name: String,
	source: Source,
	key: String,
	srd: bool,
	basic_rules: bool,
	time: Time,
	entries: Vec<Entry>,
	see_also_action: Vec<String>,
	from_variant: String
}

impl Action {
	pub fn new(object: Option<&Value>) -> Action {
		let p_object = serde_as_object(object);
		let name = serde_as_string(p_object.get("name"), "N/A".to_string());
		let source = Source::new(p_object.get("source"), p_object.get("page"));

		return Action {
			key: form_key(&name, &source.name),
			name,
			source,
			srd: serde_as_bool(p_object.get("srd"), false),
			basic_rules: serde_as_bool(p_object.get("basicRules"), false),
			time: Time::new(p_object.get("time")),
			entries: serde_as_array::<Entry>(p_object.get("entries"), |i, _| Entry::new(i), Entry::default()),
			see_also_action: serde_as_array::<String>(p_object.get("seeAlsoAction"), serde_as_string, "N/A".to_string()),
			from_variant: serde_as_string(p_object.get("fromVariant"), "N/A".to_string())
		}
	}
}
