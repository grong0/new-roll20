use serde_json::{Map, Value};

use crate::serde_utils::{serde_as_array, serde_as_array_mapping, serde_as_bool, serde_as_object, serde_as_object_from_option, serde_as_string, serde_as_u64};

use super::common::{form_key, AdditionalSpells, ClassStartingEquipment, ClassTableGroup, Die, Entry, Multiclassing, OptionalFeatureProgression, Proficiencies, Source};

// TODO: make sure it works with other classes besides artificer
#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub source: Source,
    pub key: String,
    pub other_sources: Vec<Source>,
    pub hit_die: Die,
    pub proficiency: Vec<String>,
    pub spell_casing_ability: u64,
    pub caster_progression: String,
    pub preparsed_spells_formula: String,
    pub preparsed_spells_change: String,
    pub cantrip_progression: Vec<u64>,
    pub optional_feature_progressions: OptionalFeatureProgression,
    pub starting_proficiencies: Proficiencies,
    pub starting_equipment: ClassStartingEquipment,
    pub multiclassing: Multiclassing,
    pub class_table_groups: Vec<ClassTableGroup>,
    pub class_features: Vec<OptionalFeatureProgression>,
    pub subclass_title: String,
    pub has_fluff: bool,
    pub has_fluff_images: bool,
}

impl Class {
    pub fn new(value: Value) -> Class {
        let object = serde_as_object(&value, Map::new());

        let name = serde_as_string(object.get("name"), "N/A".to_string());
        let source = Source::new(object.get("source"), object.get("page"));

        return Class {
            key: form_key(&name, &source.name),
            name,
            source,
            other_sources: serde_as_array_mapping(object.get("otherSource"), serde_as_object_from_option, Map::new()).iter().map(|i| Source::new(i.get("source"), i.get("page"))).collect(),
            hit_die: Die::new_from_hit_die(object.get("hd")),
            proficiency: serde_as_array_mapping(object.get("proficiency"), serde_as_string, "N/A".to_string()),
            spell_casing_ability: serde_as_u64(object.get("spellCastingAbility"), 0),
            caster_progression: serde_as_string(object.get("casterProgression"), "N/A".to_string()),
            preparsed_spells_formula: serde_as_string(object.get("preparsedSpellsFormula"), "N/A".to_string()),
            preparsed_spells_change: serde_as_string(object.get("preparsedSpellsChange"), "N/A".to_string()),
            cantrip_progression: serde_as_array_mapping(object.get("cantripProgression"), serde_as_u64, 0),
            optional_feature_progressions: OptionalFeatureProgression::new(object.get("optionalFeatureProgression")),
            starting_proficiencies: Proficiencies::new(object.get("startingProficiencies")),
            starting_equipment: ClassStartingEquipment::new(object.get("startingEquipment")),
            multiclassing: Multiclassing::new(object.get("multiclassing")),
            class_table_groups: serde_as_array(object.get("classTableGroups")).iter().map(|i| ClassTableGroup::new(i)).collect(),
            class_features: serde_as_array(object.get("optionalFeatureProgression")).iter().map(|i| OptionalFeatureProgression::new(Some(i))).collect(),
            subclass_title: serde_as_string(object.get("subclassTitle"), "N/A".to_string()),
            has_fluff: serde_as_bool(object.get("hasFluff"), false),
            has_fluff_images: serde_as_bool(object.get("hasFluffImages"), false),
        };
    }
}

// TODO: make sure it works with other classes besides artificer
#[derive(Debug)]
pub struct Subclass {
    pub name: String,
    pub source: Source,
    pub key: String,
    pub short_name: String,
    pub class_source: String,
    pub other_sources: Vec<Source>,
    pub additional_spells: Vec<AdditionalSpells>,
    pub subclass_features: Vec<String>,
    pub has_fluff_images: bool,
}

impl Subclass {
    pub fn new(value: Value) -> Subclass {
        let object = serde_as_object(&value, Map::new());

        let name = serde_as_string(object.get("name"), "N/A".to_string());
        let source = Source::new(object.get("source"), object.get("page"));

        return Subclass {
            key: form_key(&name, &source.name),
            name,
            source,
            short_name: serde_as_string(object.get("shortName"), "N/A".to_string()),
            class_source: serde_as_string(object.get("classSource"), "N/A".to_string()),
            other_sources: serde_as_array_mapping(object.get("otherSource"), serde_as_object_from_option, Map::new()).iter().map(|i| Source::new(i.get("source"), i.get("page"))).collect(),
            additional_spells: serde_as_array(object.get("additionalSpells")).iter().map(|i| AdditionalSpells::new(Some(i))).collect(),
            subclass_features: serde_as_array_mapping(object.get("subclassFeatures"), serde_as_string, "N/A".to_string()),
            has_fluff_images: serde_as_bool(object.get("hasFluffImages"), false),
        };
    }
}

// TODO: make sure it works with other classes besides artificer
#[derive(Debug)]
pub struct ClassFeature {
    pub name: String,
    pub source: Source,
    pub key: String,
    pub other_sources: Vec<Source>,
    pub class_name: String,
    pub class_source: String,
    pub level: u64,
    pub entries: Vec<Entry>,
    pub header: u64,
}

impl ClassFeature {
    pub fn new(value: Value) -> ClassFeature {
        let object = serde_as_object(&value, Map::new());

        let name = serde_as_string(object.get("name"), "N/A".to_string());
        let source = Source::new(object.get("source"), object.get("page"));

        return ClassFeature {
            key: form_key(&name, &source.name),
            name,
            source,
            other_sources: serde_as_array_mapping(object.get("otherSource"), serde_as_object_from_option, Map::new()).iter().map(|i| Source::new(i.get("source"), i.get("page"))).collect(),
            class_name: serde_as_string(object.get("className"), "N/A".to_string()),
            class_source: serde_as_string(object.get("classSource"), "N/A".to_string()),
            level: serde_as_u64(object.get("level"), 0),
            entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
            header: serde_as_u64(object.get("header"), 0),
        };
    }
}

// TODO: make sure it works with other classes besides artificer
#[derive(Debug)]
pub struct SubclassFeature {
    pub name: String,
    pub source: Source,
    pub key: String,
    pub other_sources: Vec<Source>,
    pub class_name: String,
    pub class_source: String,
    pub subclass_short_name: String,
    pub subclass_source: String,
    pub level: u64,
    pub entries: Vec<Entry>,
    pub header: u64,
    pub subclass_feature_type: String,
}

impl SubclassFeature {
    pub fn new(value: Value) -> SubclassFeature {
        let object = serde_as_object(&value, Map::new());

        let name = serde_as_string(object.get("name"), "N/A".to_string());
        let source = Source::new(object.get("source"), object.get("page"));

        return SubclassFeature {
            key: form_key(&name, &source.name),
            name,
            source,
            other_sources: serde_as_array_mapping(object.get("otherSource"), serde_as_object_from_option, Map::new()).iter().map(|i| Source::new(i.get("source"), i.get("page"))).collect(),
            class_name: serde_as_string(object.get("className"), "N/A".to_string()),
            class_source: serde_as_string(object.get("classSource"), "N/A".to_string()),
            subclass_short_name: serde_as_string(object.get("subclassShortName"), "N/A".to_string()),
            subclass_source: serde_as_string(object.get("subclassSource"), "N/A".to_string()),
            level: serde_as_u64(object.get("level"), 0),
            entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
            header: serde_as_u64(object.get("header"), 0),
            subclass_feature_type: serde_as_string(object.get("type"), "N/A".to_string()),
        };
    }
}
