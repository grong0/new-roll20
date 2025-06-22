use super::common::{ArmorProficiencies, ClassWeaponProficiencies, HitDie, MulticlassingRequirements, OptionalClassFeatureProgression, SkillProficiencies, Source, StartingEquipment, ToolProficiencies};

pub struct Proficiencies {
	pub armor: ArmorProficiencies, // TODO: check to see if this is valid for the data
	pub weapons: ClassWeaponProficiencies,
	pub tools: Vec<String>,
	pub tools_proficiencies: ToolProficiencies, // TODO: check to see if this is valid for the data
	pub skills: SkillProficiencies // TODO: check to see if this is valid for the data
}

pub struct ClassStartingEquipment {
	pub additional_from_background: bool,
	pub default: Vec<String>,
	pub gold_alternative: String,
	pub default_data: StartingEquipment
}

pub struct Multiclassing {
	pub requirements: MulticlassingRequirements,
	pub proficiencies_gained: Proficiencies
}

pub struct Class {
	name: String,
	source: Source,
	key: String,
	other_sources: Vec<Source>,
	hd: HitDie,
	proficiency: Vec<String>,
	spell_casing_ability: u64,
	caster_progression: String,
	preparsed_spells_formula: String,
	preparsed_spells_change: String,
	cantrip_progression: Vec<u64>,
	optional_feature_progressions: OptionalClassFeatureProgression,
	starting_proficiencies: Proficiencies,
	starting_equipment: ClassStartingEquipment,
	multiclassing: Multiclassing,
	class_table_groups: ,
	class_features: Vec<ClassFeature>,
	subclass_title: ,
	has_fluff: ,
	has_fluff_images:
}
