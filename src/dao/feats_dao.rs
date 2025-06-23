use serde_json::{Map, Value};

use super::common::{
    form_key, serde_as_array, serde_as_bool, serde_as_object, serde_as_string, Ability, AdditionalSpells, ArmorProficiencies, Entry, Expertise, LanguageProficiencies, OptionalFeatureProgression, Prerequisite, Resist, SavingThrowProficiencies, SkillProficiencies, SkillToolLanguageProficiencies,
    Source, ToolProficiencies, WeaponProficiencies,
};

#[derive(Debug)]
pub struct Feat {
    pub name: String,
    pub source: Source,
    pub key: String,
    pub srd: bool,
    pub prerequisite: Prerequisite,
    pub ability: Ability,
    pub additional_spells: Vec<AdditionalSpells>,
    pub entries: Vec<Entry>,
    pub has_fluff_images: bool,
    pub tool_proficiencies: ToolProficiencies,
    pub optional_feature_progression: OptionalFeatureProgression,
    pub resist: Resist,
    pub language_proficiencies: LanguageProficiencies,
    pub weapon_proficiencies: WeaponProficiencies,
    pub armor_proficiencies: ArmorProficiencies,
    pub skill_proficiencies: SkillProficiencies,
    pub saving_throw_proficiencies: SavingThrowProficiencies,
    pub expertise: Expertise,
    pub skill_tool_language_proficiencies: SkillToolLanguageProficiencies,
}

impl Feat {
    pub fn new(value: Value) -> Feat {
        let object = serde_as_object(&value, Map::new());

        let name = serde_as_string(object.get("name"), "N/A".to_string());
        let source = Source::new(object.get("source"), object.get("page"));

        return Feat {
            key: form_key(&name, &source.name),
            name,
            source,
            srd: serde_as_bool(object.get("srd"), false),
            prerequisite: Prerequisite::new(object.get("prerequisite")),
            ability: Ability::new(object.get("ability")),
            additional_spells: serde_as_array(p_object.get("additionalSpells")).iter().map(|i| AdditionalSpells::new(Some(i))).collect(),
            entries: serde_as_array(object.get("entries")).iter().map(|i| Entry::new(i)).collect(),
            has_fluff_images: serde_as_bool(object.get("hasFluffImages"), false),
            tool_proficiencies: ToolProficiencies::new(object.get("toolProficiencies")),
            optional_feature_progression: OptionalFeatureProgression::new(object.get("optionalFeatureProgression")),
            resist: Resist::new(object.get("resist")),
            language_proficiencies: LanguageProficiencies::new(object.get("languageProficiencies")),
            weapon_proficiencies: WeaponProficiencies::new(object.get("weaponProficiencies")),
            armor_proficiencies: ArmorProficiencies::new(object.get("armorProficiencies")),
            skill_proficiencies: SkillProficiencies::new(object.get("skillProficiencies")),
            saving_throw_proficiencies: SavingThrowProficiencies::new(object.get("savingThrowProficiencies")),
            expertise: Expertise::new(object.get("expertise")),
            skill_tool_language_proficiencies: SkillToolLanguageProficiencies::new(object.get("skillToolLanguageProficiencies")),
        };
    }
}
