use serde_json::{Map, Value};

use super::common::{form_key, serde_as_array, serde_as_array_mapping, serde_as_bool, serde_as_object, serde_as_string, Entry, Source, Time};

#[derive(Debug)]
pub struct Action {
    pub name: String,
    pub source: Source,
    pub key: String,
    pub srd: bool,
    pub basic_rules: bool,
    pub time: Time,
    pub entries: Vec<Entry>,
    pub see_also_action: Vec<String>,
    pub from_variant: String,
}

impl Action {
    pub fn new(object: Value) -> Action {
        let p_object = serde_as_object(&object, Map::new());
        let name = serde_as_string(p_object.get("name"), "N/A".to_string());
        let source = Source::new(p_object.get("source"), p_object.get("page"));

        return Action {
            key: form_key(&name, &source.name),
            name,
            source,
            srd: serde_as_bool(p_object.get("srd"), false),
            basic_rules: serde_as_bool(p_object.get("basicRules"), false),
            time: Time::new(p_object.get("time")),
            entries: serde_as_array(p_object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
            see_also_action: serde_as_array_mapping::<String>(p_object.get("seeAlsoAction"), serde_as_string, "N/A".to_string()),
            from_variant: serde_as_string(p_object.get("fromVariant"), "N/A".to_string()),
        };
    }
}
