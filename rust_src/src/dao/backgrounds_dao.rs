use serde_json::{Map, Value};

use super::common::{
    form_key, serde_as_array, serde_as_array_mapping, serde_as_bool, serde_as_object, serde_as_object_from_option, serde_as_string, AdditionalSpells, Entry, LanguageProficiencies, Prerequisite, SkillProficiencies, SkillToolLanguageProficiencies, Source, StartingEquipment, ToolProficiencies,
};

/**
 * Has a _copy
 */
#[derive(Debug)]
pub struct Background {
    name: String,
    source: Source,
    key: String,
    srd: bool,
    basic_rules: bool,
    skill_proficiencies: SkillProficiencies,
    language_proficiencies: LanguageProficiencies,
    starting_equipment: StartingEquipment,
    entries: Vec<Entry>,
    has_fluff: bool,
    tool_proficiencies: ToolProficiencies,
    feats: Vec<String>,
    from_feature: Vec<String>,
    has_fluff_images: bool,
    additional_spells: AdditionalSpells,
    additional_sources: Vec<Source>,
    prerequisite: Prerequisite,
    skill_tool_language_proficiencies: SkillToolLanguageProficiencies,
    other_sources: Vec<Source>,
}

impl Background {
    pub fn new(object: Value) -> Background {
        let p_object = serde_as_object(&object, Map::new());

        let name = serde_as_string(p_object.get("name"), "N/A".to_string());
        let source = Source::new(p_object.get("source"), p_object.get("page"));

        return Background {
            key: form_key(&name, &source.name),
            name,
            source,
            srd: serde_as_bool(p_object.get("srd"), false),
            basic_rules: serde_as_bool(p_object.get("basicRules"), false),
            skill_proficiencies: SkillProficiencies::new(p_object.get("skillProficiencies")),
            language_proficiencies: LanguageProficiencies::new(p_object.get("languageProficiencies")),
            starting_equipment: StartingEquipment::new(p_object.get("startingEquipment")),
            entries: serde_as_array(p_object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
            has_fluff: serde_as_bool(p_object.get("hasFluff"), false),
            tool_proficiencies: ToolProficiencies::new(p_object.get("toolProficiencies")),
            feats: serde_as_array_mapping(p_object.get("feats"), serde_as_string, "N/A".to_string()),
            from_feature: serde_as_array_mapping(p_object.get("fromFeature"), serde_as_string, "N/A".to_string()),
            has_fluff_images: serde_as_bool(p_object.get("hasFluffImages"), false),
            additional_spells: AdditionalSpells::new(p_object.get("additionalSpells")),
            additional_sources: serde_as_array_mapping(p_object.get("additionalSources"), serde_as_object_from_option, Map::new()).iter().map(|i| Source::new(i.get("source"), i.get("page"))).collect(),
            prerequisite: Prerequisite::new(p_object.get("prerequisite")),
            skill_tool_language_proficiencies: SkillToolLanguageProficiencies::new(p_object.get("skillToolLanguageProficiencies")),
            other_sources: serde_as_array_mapping(p_object.get("otherSources"), serde_as_object_from_option, Map::new()).iter().map(|i| Source::new(i.get("source"), i.get("page"))).collect(),
        };
    }
}
