use serde_json::{Map, Value};

use super::common::{form_key, serde_as_array_mapping, serde_as_bool, serde_as_object_from_option, serde_as_string, Source};

#[derive(Debug)]
pub struct Language {
    name: String,
    source: Source,
    key: String,
    typical_speakers: Vec<String>,
    language_type: String,
    script: String,
    srd: bool,
    basic_rules: bool,
    entries: Vec<String>,
    has_fluff_images: bool,
    fonts: Vec<String>,
    additional_sources: Vec<Source>,
    dialects: Vec<String>,
    other_sources: Vec<Source>,
}

impl Language {
    pub fn new(value: Option<&Value>) -> Language {
        let object = serde_as_object_from_option(value, Map::new());

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
            entries: serde_as_array_mapping(object.get("entries"), serde_as_string, "N/A".to_string()),
            has_fluff_images: serde_as_bool(object.get("hasFluffImages"), false),
            fonts: serde_as_array_mapping(object.get("fonts"), serde_as_string, "N/A".to_string()),
            additional_sources: serde_as_array_mapping(object.get("additionalSources"), serde_as_object_from_option, Map::new()).iter().map(|i| Source::new(i.get("source"), i.get("page"))).collect(),
            dialects: serde_as_array_mapping(object.get("dialects"), serde_as_string, "N/A".to_string()),
            other_sources: serde_as_array_mapping(object.get("otherSources"), serde_as_object_from_option, Map::new()).iter().map(|i| Source::new(i.get("source"), i.get("page"))).collect(),
        };
    }
}
