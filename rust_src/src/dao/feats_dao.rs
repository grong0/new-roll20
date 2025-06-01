use serde_json::{Map, Value};

use super::common::{form_key, serde_as_array, serde_as_bool, serde_as_object, serde_as_string, Ability, AdditionalSpells, ArmorProficiencies, Entry, Expertise, LanguageProficiencies, OptionalFeatureProgression, Prerequisite, Resist, SavingThrowProficiencies, SkillProficiencies, SkillToolLanguageProficiencies, Source, ToolProficiencies, WeaponProficiencies};

#[derive(Debug)]
pub struct Feat {
	name: String,
    source: Source,
	key: String,
    srd: bool,
    prerequisite: Prerequisite,
    ability: Ability,
    additional_spells: AdditionalSpells,
    entries: Vec<Entry>,
    has_fluff_images: bool,
    tool_proficiencies: ToolProficiencies,
    optional_feature_progression: OptionalFeatureProgression,
    resist: Resist,
    language_proficiencies: LanguageProficiencies,
    weapon_proficiencies: WeaponProficiencies,
    armor_proficiencies: ArmorProficiencies,
    skill_proficiencies: SkillProficiencies,
    saving_throw_proficiencies: SavingThrowProficiencies,
    expertise: Expertise,
    skill_tool_language_proficiencies: SkillToolLanguageProficiencies,
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
			additional_spells: AdditionalSpells::new(object.get("additionalSpells")),
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
		}
	}
}
