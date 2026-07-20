use crate::dao::{utils::{Page, Prerequisite, ReprintedAs, SkillProficiencies, Source, ToolProficiencies, deserialize_page, deserialize_reprinted_as}, utils_edition::Edition};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Background {
	#[serde(default)]
	pub name: String,
	#[serde(default)]
	pub source: Source,
	#[serde(default, deserialize_with = "deserialize_page")]
	pub page: Page,
	#[serde(default, deserialize_with = "deserialize_reprinted_as")]
	pub reprinted_as: ReprintedAs,
	#[serde(default)]
	pub edition: Edition,
	#[serde(default)]
	pub prerequisite: Vec<Prerequisite>,
	#[serde(default)]
	pub skill_proficiencies: SkillProficiencies,
	#[serde(default)]
	pub tool_proficiencies: ToolProficiencies,
	// #[serde(default)]
	// pub language_proficiencies: LanguageProficiencies,
	// #[serde(default)]
	// pub skill_tool_language_proficiencies: SkillToolLanguageProficiencies,
	// #[serde(default)]
	// pub weapon_proficiencies: WeaponProficiencies,
	// #[serde(default)]
	// pub armor_proficiencies: ArmorProficiencies,
	// #[serde(default)]
	// pub feats: AdditionalFeatsArray,
	// #[serde(default)]
	// pub entries: Vec<Entry>,
	// #[serde(default)]
	// pub additional_sources: AdditionalSources,
	// #[serde(default)]
	// pub other_sources: OtherSources,
	// #[serde(default)]
	// pub reference_sources: ReferenceSources,
	// #[serde(default)]
	// pub starting_equipment: StartingEquipment,
	// #[serde(default)]
	// pub additional_spells: AdditionalSpellsArray,
	// #[serde(default)]
	// pub ability: AbilityScores,
	// #[serde(default)]
	// /* A lookup of other properties which should be tied to the "Feature: ..." entry. This is used when e.g. customizing a background during import to a VTT. */
	// pub from_feature: FromFeature,
	// #[serde(default = false)]
	// pub has_fluff: bool,
	// #[serde(default = false)]
	// pub has_fluff_images: bool,
	// #[serde(default)]
	// pub srd: SRD,
	// #[serde(default)]
	// pub srd_52: SRD,
	// #[serde(default)]
	// pub basic_rules: BasicRules,
	// #[serde(default)]
	// pub basic_rules_2024: BasicRules,
}
