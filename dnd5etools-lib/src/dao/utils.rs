use std::collections::HashSet;

use serde::{
	Deserialize, Deserializer,
	de::{self, Visitor},
};

use crate::dao::{
	items_shared::{ItemPropertyArray, ItemType},
	utils_edition::Edition,
};

pub type MetaDependenciesArray = HashSet<String>;

/** "uri-reference" is unsuitable for brew, as it requires URL-encoding "'" (see: https://www.ietf.org/rfc/rfc3986.txt) */
type SourceString = String;

pub type Source = SourceString;

pub type Page = u64;

pub fn deserialize_page<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
	D: Deserializer<'de>,
{
	pub struct PageVisitor;

	impl<'de> Visitor<'de> for PageVisitor {
		type Value = u64;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("an i64, or roman numerals as a string");
		}

		fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			Ok(v)
		}

		fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			println!("got string: {}", v);
			let mut total = 0;
			for roman_numeral in v.chars() {
				match roman_numeral {
					'i' => total += 1,
					'v' => total += 5,
					'x' => total += 10,
					_ => total += 0,
				}
			}
			println!("roman_numerals_parser was called and turned '{}' into '{}'", v, total);
			Ok(total)
		}
	}

	return deserializer.deserialize_any(PageVisitor);
}

pub type Alias = Vec<String>;

#[derive(Debug, Default, Deserialize)]
pub struct ReprintedAs {
	/** A UID, e.g. \"longsword|phb\" */
	uid: String,
	/** A tag used in the renderer, e.g. \"creature\" */
	tag: String,
	edition: Edition,
}

pub fn deserialize_reprinted_as<'de, D>(deserializer: D) -> Result<ReprintedAs, D::Error>
where
	D: Deserializer<'de>,
{
	struct ReprintedAsVisitor;

	impl<'de> Visitor<'de> for ReprintedAsVisitor {
		type Value = ReprintedAs;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("A UID, e.g. \"longsword|phb\", or a ReprintedAs object");
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			println!("visit_string was called!");
			Ok(ReprintedAs { uid: String::from(v), ..Default::default() })
		}
	}

	return deserializer.deserialize_any(ReprintedAsVisitor);
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum TagNameStats {
	#[serde(rename = "spell")]
	Spell,
	#[serde(rename = "item")]
	Item,
	#[serde(rename = "itemMastery")]
	ItemMastery,
	#[serde(rename = "creature")]
	Creature,
	#[serde(rename = "condition")]
	Condition,
	#[serde(rename = "disease")]
	Disease,
	#[serde(rename = "status")]
	Status,
	#[serde(rename = "background")]
	Background,
	#[serde(rename = "race")]
	Race,
	#[serde(rename = "optfeature")]
	Optfeature,
	#[serde(rename = "reward")]
	Reward,
	#[serde(rename = "feat")]
	Feat,
	#[serde(rename = "psionic")]
	Psionic,
	#[serde(rename = "object")]
	Object,
	#[serde(rename = "cult")]
	Cult,
	#[serde(rename = "boon")]
	Boon,
	#[serde(rename = "trap")]
	Trap,
	#[serde(rename = "hazard")]
	Hazard,
	#[serde(rename = "variantrule")]
	Variantrule,
	#[serde(rename = "charoption")]
	Charoption,
	#[serde(rename = "vehicle")]
	Vehicle,
	#[serde(rename = "vehupgrade")]
	Vehupgrade,
	#[serde(rename = "class")]
	Class,
	#[serde(rename = "action")]
	Action,
	#[serde(rename = "language")]
	Language,
	#[serde(rename = "classFeature")]
	ClassFeature,
	#[serde(rename = "subclassFeature")]
	SubclassFeature,
	#[serde(rename = "table")]
	Table,
	#[serde(rename = "recipe")]
	Recipe,
	#[serde(rename = "skill")]
	Skill,
	#[serde(rename = "sense")]
	Sense,
	#[serde(rename = "legroup")]
	Legroup,
	#[serde(rename = "deck")]
	Deck,
	#[serde(rename = "facility")]
	Facility,
	#[default]
	None,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum AbilityScoreAbbreviation {
	#[serde(rename = "str")]
	Str,
	#[serde(rename = "dex")]
	Dex,
	#[serde(rename = "con")]
	Con,
	#[serde(rename = "int")]
	Int,
	#[serde(rename = "wis")]
	Wis,
	#[serde(rename = "cha")]
	Cha,
	#[default]
	None
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrerequisiteLevelClass {
	name: String,
	source: Source,
	/** Governs whether or not the class name is visible in the list display/prerequisite line. *Not* recommended for features which implicitly carry a class restriction, such as Eldritch Invocations. This functions as a combination of \"visibleStats\" and \"visibleList\". */
	visible: bool,
	/** Governs whether or not the class name is visible in the prerequisite line. */
	visible_stats: bool,
	/** Governs whether or not the class name is visible in the list display. *Not* recommended for features which implicitly carry a class restriction, such as Eldritch Invocations. */
	visible_list: bool,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrerequisiteLevelSubclass {
	name: String,
	source: Source,
	/** Governs whether or not the class name is visible in the list display/prerequisite line. *Not* recommended for features which implicitly carry a class restriction, such as Eldritch Invocations. This functions as a combination of \"visibleStats\" and \"visibleList\". */
	visible: bool,
	/** Governs whether or not the class name is visible in the prerequisite line. */
	visible_stats: bool,
	/** Governs whether or not the class name is visible in the list display. *Not* recommended for features which implicitly carry a class restriction, such as Eldritch Invocations. */
	visible_list: bool,
}

#[derive(Debug, Default, Deserialize)]
struct PrerequisiteLevel {
	level: u64,
	class: PrerequisiteLevelClass,
	subclass: PrerequisiteLevelSubclass,
}

/**
 * TODO: Make sure this also deserializes the object.
 */
pub fn deserialize_prerequisite_level<'de, D>(deserializer: D) -> Result<PrerequisiteLevel, D::Error>
where
	D: Deserializer<'de>,
{
	struct PrerequisiteLevelVisitor;

	impl<'de> Visitor<'de> for PrerequisiteLevelVisitor {
		type Value = PrerequisiteLevel;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("A UID, e.g. \"longsword|phb\", or a ReprintedAs object");
		}

		fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			Ok(PrerequisiteLevel { level: v, ..Default::default() })
		}
	}

	return deserializer.deserialize_any(PrerequisiteLevelVisitor);
}

#[derive(Debug, Default, Deserialize)]
enum PrerequisitePact {
	Chain,
	Tome,
	Blade,
	Talisman,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
enum PrerequisitePatron {
	#[serde(rename = "The Archfey")]
	TheArchfey,
	#[serde(rename = "The Fiend")]
	TheFiend,
	#[serde(rename = "The Great Old One")]
	TheGreatOldOne,
	#[serde(rename = "The Hexblade")]
	TheHexblade,
	#[serde(rename = "The Raven Queen")]
	TheRavenQueen,
	#[serde(rename = "The Seeker")]
	TheSeeker,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrerequisiteSpell {
	/** A range prerequisite spells, formatted similarly to the options in a {@filter ...} tag.\n\nFor example: \"level=0|class=Wizard\" */
	choose: String,
	entry: String,
	/** Used in short/list displays */
	entry_summary: String,
}

pub fn deserialize_prerequisite_spell<'de, D>(deserializer: D) -> Result<PrerequisiteSpell, D::Error>
where
	D: Deserializer<'de>,
{
	struct PrerequisiteSpellVisitor;

	impl<'de> Visitor<'de> for PrerequisiteSpellVisitor {
		type Value = PrerequisiteSpell;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			// TODO: make expecting text
			return formatter.write_str("");
		}

		fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			Ok(PrerequisiteSpell { choose: v, ..Default::default() })
		}
	}

	return deserializer.deserialize_any(PrerequisiteSpellVisitor);
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtherSummary {
	entry: String,
	/** Used in short/list displays */
	entry_summary: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrerequisiteRace {
	name: String,
	/** Optional long-form name to be used in the rendered entity. */
	display_entry: String,
	subrace: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrerequisiteBackground {
	name: String,
	/** Optional long-form name to be used in the rendered entity. */
	display_entry: String,
}

#[derive(Debug, Default, Deserialize)]
struct PrerequisiteAbility {
	str: i64,
	dex: i64,
	con: i64,
	int: i64,
	wis: i64,
	cha: i64,
}

// TODO: remove default
#[derive(Debug, Default)]
enum PrerequisiteSpellcastingFocusTool {
	Arcane,
	Druid,
	Holy,
	Tool,
	ArtisansTool,
	#[default]
	None,
}

#[derive(Debug, Default)]
struct PrerequisiteSpellcastingFocus {
	needs_focus: bool,
	focus_tools: Vec<PrerequisiteSpellcastingFocusTool>,
}

pub fn deserialize_prerequisite_spellcasting_focus<'de, D>(deserializer: D) -> Result<PrerequisiteSpellcastingFocus, D::Error>
where
	D: Deserializer<'de>,
{
	struct PrerequisiteSpellcastingFocusVisitor;

	impl<'de> Visitor<'de> for PrerequisiteSpellcastingFocusVisitor {
		type Value = PrerequisiteSpellcastingFocus;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("A UID, e.g. \"longsword|phb\", or a ReprintedAs object");
		}

		fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			Ok(PrerequisiteSpellcastingFocus { needs_focus: true, focus_tools: vec![] })
		}

		fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
		where
			A: de::SeqAccess<'de>,
		{
			let mut focus_tools: Vec<PrerequisiteSpellcastingFocusTool> = vec![];
			let element = seq.next_element()?;
			while element.is_some() {
				let element_value: &str = element.unwrap();
				focus_tools.push(match element_value {
					"arcane" => PrerequisiteSpellcastingFocusTool::Arcane,
					"druid" => PrerequisiteSpellcastingFocusTool::Druid,
					"holy" => PrerequisiteSpellcastingFocusTool::Holy,
					"tool" => PrerequisiteSpellcastingFocusTool::Tool,
					"artisansTool" => PrerequisiteSpellcastingFocusTool::ArtisansTool,
					_ => PrerequisiteSpellcastingFocusTool::None,
				});
			}
			Ok(PrerequisiteSpellcastingFocus { needs_focus: true, focus_tools: focus_tools })
		}
	}

	return deserializer.deserialize_any(PrerequisiteSpellcastingFocusVisitor);
}

#[derive(Debug, Default, Deserialize)]
enum PrerequisiteProficiencyArmor {
	Simple,
	Martial,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
enum PrerequisiteProficiencyWeaponGroup {
	Simple,
	Martial,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PrerequisiteProficiency {
	/** Any <simple|martial> weapon. */
	armor: PrerequisiteProficiencyArmor,
	weapon: String,
	/** All <simple|martial> weapons. */
	weapon_group: PrerequisiteProficiencyWeaponGroup,
}

/**
 * Check to see if the source is a hash map where the keys are different skills and not just the one key "skill".
 */
#[derive(Debug, Default, Deserialize)]
struct PrerequisteExpertise {
	skill: bool,
}

#[derive(Debug, Default, Deserialize)]
enum PrerequisiteCampaign {
	Dragonlance,
	Planescape,
	Eberron,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
enum PrerequisiteMembership {
	CultOfTheDragon,
	EmeraldEnclave,
	Harpers,
	LordsAlliance,
	OrderOfTheGauntlet,
	PurpleDragonKnights,
	RedWizards,
	Zhentarim,
	#[default]
	None,
}

pub fn deserialize_prerequisite_membership<'de, D>(deserializer: D) -> Result<PrerequisiteMembership, D::Error>
where
	D: Deserializer<'de>,
{
	struct PrerequisiteMembershipVisitor;

	impl<'de> Visitor<'de> for PrerequisiteMembershipVisitor {
		type Value = PrerequisiteMembership;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			match v {
				"Cult of the Dragon" => Ok(PrerequisiteMembership::CultOfTheDragon),
				"Emerald Enclave" => Ok(PrerequisiteMembership::EmeraldEnclave),
				"Harpers" => Ok(PrerequisiteMembership::Harpers),
				"Lords' Alliance" => Ok(PrerequisiteMembership::LordsAlliance),
				"Order of the Gauntlet" => Ok(PrerequisiteMembership::OrderOfTheGauntlet),
				"Purple Dragon Knights" => Ok(PrerequisiteMembership::PurpleDragonKnights),
				"Red Wizards" => Ok(PrerequisiteMembership::RedWizards),
				"Zhentarim" => Ok(PrerequisiteMembership::Zhentarim),
				_ => Err(E::custom("not a valid membership")),
			}
		}
	}

	return deserializer.deserialize_str(PrerequisiteMembershipVisitor);
}

pub fn deserialize_prerequisite_membership_array<'de, D>(deserializer: D) -> Result<Vec<PrerequisiteMembership>, D::Error>
where
	D: Deserializer<'de>,
{
	struct PrerequisiteMembershipArrayVisitor;

	impl<'de> Visitor<'de> for PrerequisiteMembershipArrayVisitor {
		type Value = Vec<PrerequisiteMembership>;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
		where
			A: de::SeqAccess<'de>,
		{
			let mut values = vec![];
			let element = seq.next_element()?;
			while element.is_some() {
				let element_value: &str = element.unwrap();
				let value = match element_value {
					"Cult of the Dragon" => PrerequisiteMembership::CultOfTheDragon,
					"Emerald Enclave" => PrerequisiteMembership::EmeraldEnclave,
					"Harpers" => PrerequisiteMembership::Harpers,
					"Lords' Alliance" => PrerequisiteMembership::LordsAlliance,
					"Order of the Gauntlet" => PrerequisiteMembership::OrderOfTheGauntlet,
					"Purple Dragon Knights" => PrerequisiteMembership::PurpleDragonKnights,
					"Red Wizards" => PrerequisiteMembership::RedWizards,
					"Zhentarim" => PrerequisiteMembership::Zhentarim,
					_ => PrerequisiteMembership::None,
				};
				values.push(value);
			}
			Ok(values)
		}
	}

	return deserializer.deserialize_seq(PrerequisiteMembershipArrayVisitor);
}

#[derive(Debug, Default, Deserialize)]
enum PrerequisiteGroup {
	Expert,
	Mage,
	Priest,
	Warrior,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct Prerequisite {
	#[serde(default, deserialize_with = "deserialize_prerequisite_level")]
	level: PrerequisiteLevel,
	#[serde(default)]
	pact: PrerequisitePact,
	#[serde(default)]
	patron: PrerequisitePatron,
	#[serde(default)]
	spell: PrerequisiteSpell,
	#[serde(default)]
	feat: Vec<String>,
	#[serde(default)]
	feat_category: Vec<DataFeatCategory>,
	#[serde(default)]
	exclusive_feat_category: Vec<DataFeatCategory>,
	#[serde(default, rename = "optionalfeature")]
	optional_feature: Vec<String>,
	#[serde(default)]
	feature: Vec<String>,
	#[serde(default)]
	item: Vec<String>,
	#[serde(default)]
	item_type: Vec<ItemType>,
	#[serde(default)]
	item_property: ItemPropertyArray,
	/** A free text prerequisite */
	#[serde(default)]
	other: String,
	/** A free text prerequisite, with a shortened form for list display. */
	#[serde(default)]
	other_summary: OtherSummary,
	#[serde(default)]
	race: Vec<PrerequisiteRace>,
	#[serde(default)]
	background: Vec<PrerequisiteBackground>,
	#[serde(default)]
	ability: Vec<PrerequisiteAbility>,
	#[serde(default)]
	spellcasting: bool,
	#[serde(default)]
	/** Renders with the updated text found in UA2020: Feats */
	spellcasting2020: bool,
	#[serde(default)]
	/** A more restrictive spellcasting variant; specifically \"Spellcasting Feature\" */
	spellcasting_feature: bool,
	#[serde(default)]
	/** A more restrictive spellcasting variant; specifically \"Spellcasting feature from a class that prepares spells\" */
	spellcasting_prepared: bool,
	#[serde(default)]
	psionics: bool,
	#[serde(default, deserialize_with = "deserialize_prerequisite_spellcasting_focus")]
	spellcasting_focus: PrerequisiteSpellcastingFocus,
	#[serde(default)]
	proficiency: Vec<PrerequisiteProficiency>,
	#[serde(default)]
	expertise: Vec<PrerequisteExpertise>,
	#[serde(default)]
	alignment: Vec<Alignment>,
	#[serde(default)]
	campaign: Vec<PrerequisiteCampaign>,
	#[serde(default)]
	/** Example: "Elven" */
	culture: Vec<String>,
	#[serde(default, deserialize_with = "deserialize_prerequisite_membership_array")]
	membership: Vec<PrerequisiteMembership>,
	#[serde(default)]
	group: Vec<PrerequisiteGroup>,
	#[serde(default)]
	/** An additional note that accompanies the proficiencies, but is not part of them. */
	note: String,
}

#[derive(Debug, Default, Deserialize)]
enum Alignment {
	/** Lawful */
	L,
	/** Neutral */
	N,
	/** Neutral (law/chaos axis) */
	NX,
	/** Neutral (good/evil axis) */
	NY,
	/** Chaotic */
	C,
	/** Good */
	G,
	/** Evil */
	E,
	/** Unaligned */
	U,
	/** Any */
	#[default]
	A,
}

#[derive(Debug, Deserialize)]
pub enum LanguageNameLower {
	#[serde(rename = "abyssal")]
	Abyssal,
	#[serde(rename = "aquan")]
	Aquan,
	#[serde(rename = "auran")]
	Auran,
	#[serde(rename = "celestial")]
	Celestial,
	#[serde(rename = "common")]
	Common,
	#[serde(rename = "common sign language")]
	CommonSignLanguage,
	#[serde(rename = "deep speech")]
	DeepSpeech,
	#[serde(rename = "draconic")]
	Draconic,
	#[serde(rename = "druidic")]
	Druidic,
	#[serde(rename = "dwarvish")]
	Dwarvish,
	#[serde(rename = "elvish")]
	Elvish,
	#[serde(rename = "giant")]
	Giant,
	#[serde(rename = "gith")]
	Gith,
	#[serde(rename = "gnomish")]
	Gnomish,
	#[serde(rename = "goblin")]
	Goblin,
	#[serde(rename = "halfling")]
	Halfing,
	#[serde(rename = "ignan")]
	Ignan,
	#[serde(rename = "infernal")]
	Infernal,
	#[serde(rename = "orc")]
	Orc,
	#[serde(rename = "other")]
	Other,
	#[serde(rename = "primordial")]
	Primordial,
	#[serde(rename = "sylvan")]
	Sylvan,
	#[serde(rename = "terran")]
	Terran,
	#[serde(rename = "thieves' cant")]
	ThievesCant,
	#[serde(rename = "undercommon")]
	Undercommon,
}

#[derive(Debug, Default, Deserialize)]
pub struct LanguageProficiencyChoose {
	#[serde(default)]
	from: Vec<LanguageNameLower>,
	#[serde(default)]
	count: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct LanguageProficiency {
	#[serde(default)]
	any: i64,
	#[serde(default, rename = "anyStandard")]
	any_standard: i64,
	#[serde(default, rename = "anyExotic")]
	any_exoitic: i64,
	#[serde(default, rename = "anyRare")]
	any_rare: i64,
	#[serde(default)]
	abyssal: bool,
	#[serde(default)]
	celestial: bool,
	#[serde(default)]
	common: bool,
	#[serde(default, rename = "common sign language")]
	common_sign_language: bool,
	#[serde(default, rename = "deep speech")]
	deep_speech: bool,
	#[serde(default)]
	draconic: bool,
	#[serde(default)]
	infernal: bool,
	#[serde(default)]
	gith: bool,
	#[serde(default)]
	gnomish: bool,
	#[serde(default)]
	goblin: bool,
	#[serde(default)]
	halfling: bool,
	#[serde(default)]
	orc: bool,
	#[serde(default)]
	primordial: bool,
	#[serde(default)]
	aquan: bool,
	#[serde(default)]
	auran: bool,
	#[serde(default)]
	ignan: bool,
	#[serde(default)]
	terran: bool,
	#[serde(default)]
	sylvan: bool,
	#[serde(default)]
	undercommon: bool,
	#[serde(default)]
	dwarvish: bool,
	#[serde(default)]
	elvish: bool,
	#[serde(default)]
	giant: bool,
	#[serde(default)]
	other: bool,
	#[serde(default)]
	druidic: bool,
	#[serde(default, rename = "thieves' cant")]
	thieves_cant: bool,
	#[serde(default)]
	choose: LanguageProficiencyChoose,
}

pub type LanguageProficiencies = Vec<LanguageProficiency>;

#[derive(Debug, Deserialize)]
pub enum ToolNameLower {
	#[serde(rename = "artisan's tools")]
	ArtisansTools,
	#[serde(rename = "alchemist's supplies")]
	AlchemistsSupplies,
	#[serde(rename = "brewer's supplies")]
	BrewersSupplies,
	#[serde(rename = "calligrapher's supplies")]
	CalligraphersSupplies,
	#[serde(rename = "carpenter's tools")]
	CarpentersTools,
	#[serde(rename = "cartographer's tools")]
	CartographersTools,
	#[serde(rename = "cobbler's tools")]
	CobblersTools,
	#[serde(rename = "cook's utensils")]
	CooksUtensils,
	#[serde(rename = "glassblower's tools")]
	GlassblowersTools,
	#[serde(rename = "jeweler's tools")]
	JewelersTools,
	#[serde(rename = "leatherworker's tools")]
	LeatherWorkersTools,
	#[serde(rename = "mason's tools")]
	MasonsTools,
	#[serde(rename = "painter's supplies")]
	PaintersSupplies,
	#[serde(rename = "potter's tools")]
	PottersTools,
	#[serde(rename = "smith's tools")]
	SmithsTools,
	#[serde(rename = "tinker's tools")]
	TinkersTools,
	#[serde(rename = "weaver's tools")]
	WeaversTools,
	#[serde(rename = "woodcarver's tools")]
	WoodcarversTools,
	#[serde(rename = "disguise kit")]
	DisguiseKit,
	#[serde(rename = "forgery kit")]
	ForgeryKit,
	#[serde(rename = "gaming set")]
	GamingSet,
	#[serde(rename = "dragonchess set")]
	DragonChess,
	#[serde(rename = "dice set")]
	DiceSet,
	#[serde(rename = "three-dragon ante set")]
	ThreeDragonAnteSet,
	#[serde(rename = "playing card set")]
	PlayerCadSet,
	#[serde(rename = "herbalism kit")]
	HerbalismKit,
	#[serde(rename = "musical instrument")]
	MusicalInstrument,
	#[serde(rename = "bagpipes")]
	BagPipe,
	#[serde(rename = "drum")]
	Drum,
	#[serde(rename = "dulcimer")]
	Dulcimer,
	#[serde(rename = "flute")]
	Flute,
	#[serde(rename = "horn")]
	Horn,
	#[serde(rename = "lute")]
	Lute,
	#[serde(rename = "lyre")]
	Lyre,
	#[serde(rename = "pan flute")]
	PanFlute,
	#[serde(rename = "shawm")]
	Shawm,
	#[serde(rename = "viol")]
	Viol,
	#[serde(rename = "navigator's tools")]
	NavigatorsTools,
	#[serde(rename = "thieves' tools")]
	ThievesTools,
	#[serde(rename = "poisoner's kit")]
	PoisonersKit,
	#[serde(rename = "vehicles")]
	Vehicles,
	#[serde(rename = "vehicles (air)")]
	VehiclesAir,
	#[serde(rename = "vehicles (land)")]
	VehiclesLand,
	#[serde(rename = "vehicles (water)")]
	VehiclesWater,
	#[serde(rename = "vehicles (space)")]
	VehiclesSpace,
}

#[derive(Debug, Deserialize)]
pub enum ToolProficiencyChooseFrom {
	#[serde(rename = "artisan's tools")]
	ArtisansTools,
	#[serde(rename = "alchemist's supplies")]
	AlchemistsSupplies,
	#[serde(rename = "brewer's supplies")]
	BrewersSupplies,
	#[serde(rename = "calligrapher's supplies")]
	CalligraphersSupplies,
	#[serde(rename = "carpenter's tools")]
	CarpentersTools,
	#[serde(rename = "cartographer's tools")]
	CartographersTools,
	#[serde(rename = "cobbler's tools")]
	CobblersTools,
	#[serde(rename = "cook's utensils")]
	CooksUtensils,
	#[serde(rename = "glassblower's tools")]
	GlassblowersTools,
	#[serde(rename = "jeweler's tools")]
	JewelersTools,
	#[serde(rename = "leatherworker's tools")]
	LeatherWorkersTools,
	#[serde(rename = "mason's tools")]
	MasonsTools,
	#[serde(rename = "painter's supplies")]
	PaintersSupplies,
	#[serde(rename = "potter's tools")]
	PottersTools,
	#[serde(rename = "smith's tools")]
	SmithsTools,
	#[serde(rename = "tinker's tools")]
	TinkersTools,
	#[serde(rename = "weaver's tools")]
	WeaversTools,
	#[serde(rename = "woodcarver's tools")]
	WoodcarversTools,
	#[serde(rename = "disguise kit")]
	DisguiseKit,
	#[serde(rename = "forgery kit")]
	ForgeryKit,
	#[serde(rename = "gaming set")]
	GamingSet,
	#[serde(rename = "dragonchess set")]
	DragonChess,
	#[serde(rename = "dice set")]
	DiceSet,
	#[serde(rename = "three-dragon ante set")]
	ThreeDragonAnteSet,
	#[serde(rename = "playing card set")]
	PlayerCadSet,
	#[serde(rename = "herbalism kit")]
	HerbalismKit,
	#[serde(rename = "musical instrument")]
	MusicalInstrument,
	#[serde(rename = "bagpipes")]
	BagPipe,
	#[serde(rename = "drum")]
	Drum,
	#[serde(rename = "dulcimer")]
	Dulcimer,
	#[serde(rename = "flute")]
	Flute,
	#[serde(rename = "horn")]
	Horn,
	#[serde(rename = "lute")]
	Lute,
	#[serde(rename = "lyre")]
	Lyre,
	#[serde(rename = "pan flute")]
	PanFlute,
	#[serde(rename = "shawm")]
	Shawm,
	#[serde(rename = "viol")]
	Viol,
	#[serde(rename = "navigator's tools")]
	NavigatorsTools,
	#[serde(rename = "thieves' tools")]
	ThievesTools,
	#[serde(rename = "poisoner's kit")]
	PoisonersKit,
	#[serde(rename = "vehicles")]
	Vehicles,
	#[serde(rename = "vehicles (air)")]
	VehiclesAir,
	#[serde(rename = "vehicles (land)")]
	VehiclesLand,
	#[serde(rename = "vehicles (water)")]
	VehiclesWater,
	#[serde(rename = "vehicles (space)")]
	VehiclesSpace,
	#[serde(rename = "anyArtisansTool")]
	AnyArtisansTool,
	#[serde(rename = "anyMusicalInstrument")]
	AnyMusicalInstrument,
}

#[derive(Debug, Default, Deserialize)]
pub struct ToolProficiencyChoose {
	#[serde(default)]
	from: Vec<ToolProficiencyChooseFrom>,
	#[serde(default)]
	choose: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct ToolProficiency {
	#[serde(default)]
	any: i64,
	#[serde(default, rename = "anyArtisansTool")]
	any_artisans_tool: i64,
	#[serde(default, rename = "artisan's tools")]
	artisans_tools: bool,
	#[serde(default, rename = "alchemist's supplies")]
	alchemists_supplies: bool,
	#[serde(default, rename = "brewer's supplies")]
	brewers_supplies: bool,
	#[serde(default, rename = "calligrapher's supplies")]
	calligraphers_supplies: bool,
	#[serde(default, rename = "carpenter's tools")]
	carpenters_tools: bool,
	#[serde(default, rename = "cartographer's tools")]
	cartographers_tools: bool,
	#[serde(default, rename = "cobbler's tools")]
	cobblers_tools: bool,
	#[serde(default, rename = "cook's utensils")]
	cooks_utensils: bool,
	#[serde(default, rename = "glassblower's tools")]
	glassblowers_tools: bool,
	#[serde(default, rename = "jeweler's tools")]
	jewelers_tools: bool,
	#[serde(default, rename = "leatherworker's tools")]
	leather_workers_tools: bool,
	#[serde(default, rename = "mason's tools")]
	masons_tools: bool,
	#[serde(default, rename = "painter's supplies")]
	painters_supplies: bool,
	#[serde(default, rename = "potter's tools")]
	potters_tools: bool,
	#[serde(default, rename = "smith's tools")]
	smiths_tools: bool,
	#[serde(default, rename = "tinker's tools")]
	tinkers_tools: bool,
	#[serde(default, rename = "weaver's tools")]
	weavers_tools: bool,
	#[serde(default, rename = "woodcarver's tools")]
	woodcarvers_tools: bool,
	#[serde(default, rename = "disguise kit")]
	disguise_kit: bool,
	#[serde(default, rename = "forgery kit")]
	forgery_kit: bool,
	#[serde(default, rename = "anyGamingSet")]
	any_gaming_set: i64,
	#[serde(default, rename = "gaming set")]
	gaming_set: bool,
	#[serde(default, rename = "dragonchess set")]
	dragonchess_set: bool,
	#[serde(default, rename = "dice set")]
	dice_set: bool,
	#[serde(default, rename = "three-dragon ante set")]
	three_dragon_ante_set: bool,
	#[serde(default, rename = "playing card set")]
	playing_card_set: bool,
	#[serde(default, rename = "herbalism kit")]
	herbalism_kit: bool,
	#[serde(default, rename = "anyMusicalInstrument")]
	any_musical_instrument: i64,
	#[serde(default, rename = "musical instrument")]
	musical_instrument: bool,
	#[serde(default)]
	bagpipes: bool,
	#[serde(default)]
	drum: bool,
	#[serde(default)]
	dulcimer: bool,
	#[serde(default)]
	flute: bool,
	#[serde(default)]
	horn: bool,
	#[serde(default)]
	lute: bool,
	#[serde(default)]
	lyre: bool,
	#[serde(default, rename = "pan flute")]
	pan_flute: bool,
	#[serde(default)]
	shawm: bool,
	#[serde(default)]
	viol: bool,
	#[serde(default, rename = "navigator's tools")]
	navigators_tools: bool,
	#[serde(default, rename = "thieves' tools")]
	thieves_tools: bool,
	#[serde(default, rename = "poisoner's kit")]
	poisoners_kit: bool,
	#[serde(default)]
	vehicles: bool,
	#[serde(default, rename = "vehicles (air)")]
	vehicles_air: bool,
	#[serde(default, rename = "vehicles (land)")]
	vehicles_land: bool,
	#[serde(default, rename = "vehicles (water)")]
	vehicles_water: bool,
	#[serde(default, rename = "vehicles (space)")]
	vehicles_space: bool,
	#[serde(default)]
	choose: ToolProficiencyChoose,
}

pub type ToolProficiencies = Vec<ToolProficiency>;

#[derive(Debug, Deserialize)]
pub enum SkillNameLower {
	#[serde(rename = "athletics")]
	Athletics,
	#[serde(rename = "acrobatics")]
	Acrobatics,
	#[serde(rename = "sleight of hand")]
	SleightOfHand,
	#[serde(rename = "stealth")]
	Stealth,
	#[serde(rename = "arcana")]
	Arcana,
	#[serde(rename = "history")]
	History,
	#[serde(rename = "investigation")]
	Investigation,
	#[serde(rename = "nature")]
	Nature,
	#[serde(rename = "religion")]
	Religion,
	#[serde(rename = "animal handling")]
	AnimalHandling,
	#[serde(rename = "insight")]
	Insight,
	#[serde(rename = "medicine")]
	Medicine,
	#[serde(rename = "perception")]
	Perception,
	#[serde(rename = "survival")]
	Survival,
	#[serde(rename = "deception")]
	Deception,
	#[serde(rename = "intimidation")]
	Intimidation,
	#[serde(rename = "performance")]
	Performance,
	#[serde(rename = "persuasion")]
	Persuasion,
}

#[derive(Debug, Default, Deserialize)]
pub struct SkillProficiencyChoose {
	#[serde(default)]
	from: Vec<SkillNameLower>,
	#[serde(default)]
	count: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct SkillProficiency {
	#[serde(default)]
	any: i64,
	#[serde(default)]
	athletics: bool,
	#[serde(default)]
	acrobatics: bool,
	#[serde(default, rename = "sleight of hand")]
	sleight_of_hand: bool,
	#[serde(default)]
	stealth: bool,
	#[serde(default)]
	arcana: bool,
	#[serde(default)]
	history: bool,
	#[serde(default)]
	investigation: bool,
	#[serde(default)]
	nature: bool,
	#[serde(default)]
	religion: bool,
	#[serde(default, rename = "animal handling")]
	animal_handling: bool,
	#[serde(default)]
	insight: bool,
	#[serde(default)]
	medicine: bool,
	#[serde(default)]
	perception: bool,
	#[serde(default)]
	survival: bool,
	#[serde(default)]
	deception: bool,
	#[serde(default)]
	intimidation: bool,
	#[serde(default)]
	performance: bool,
	#[serde(default)]
	persuasion: bool,
	#[serde(default)]
	choose: SkillProficiencyChoose,
}

pub type SkillProficiencies = Vec<SkillProficiency>;

#[derive(Debug, Deserialize)]
pub enum SkillToolLanguageProficiencyChooseFrom {
	#[serde(rename = "abyssal")]
	Abyssal,
	#[serde(rename = "aquan")]
	Aquan,
	#[serde(rename = "auran")]
	Auran,
	#[serde(rename = "celestial")]
	Celestial,
	#[serde(rename = "common")]
	Common,
	#[serde(rename = "common sign language")]
	CommonSignLanguage,
	#[serde(rename = "deep speech")]
	DeepSpeech,
	#[serde(rename = "draconic")]
	Draconic,
	#[serde(rename = "druidic")]
	Druidic,
	#[serde(rename = "dwarvish")]
	Dwarvish,
	#[serde(rename = "elvish")]
	Elvish,
	#[serde(rename = "giant")]
	Giant,
	#[serde(rename = "gith")]
	Gith,
	#[serde(rename = "gnomish")]
	Gnomish,
	#[serde(rename = "goblin")]
	Goblin,
	#[serde(rename = "halfling")]
	Halfing,
	#[serde(rename = "ignan")]
	Ignan,
	#[serde(rename = "infernal")]
	Infernal,
	#[serde(rename = "orc")]
	Orc,
	#[serde(rename = "other")]
	Other,
	#[serde(rename = "primordial")]
	Primordial,
	#[serde(rename = "sylvan")]
	Sylvan,
	#[serde(rename = "terran")]
	Terran,
	#[serde(rename = "thieves' cant")]
	ThievesCant,
	#[serde(rename = "undercommon")]
	Undercommon,
	#[serde(rename = "artisan's tools")]
	ArtisansTools,
	#[serde(rename = "alchemist's supplies")]
	AlchemistsSupplies,
	#[serde(rename = "brewer's supplies")]
	BrewersSupplies,
	#[serde(rename = "calligrapher's supplies")]
	CalligraphersSupplies,
	#[serde(rename = "carpenter's tools")]
	CarpentersTools,
	#[serde(rename = "cartographer's tools")]
	CartographersTools,
	#[serde(rename = "cobbler's tools")]
	CobblersTools,
	#[serde(rename = "cook's utensils")]
	CooksUtensils,
	#[serde(rename = "glassblower's tools")]
	GlassblowersTools,
	#[serde(rename = "jeweler's tools")]
	JewelersTools,
	#[serde(rename = "leatherworker's tools")]
	LeatherWorkersTools,
	#[serde(rename = "mason's tools")]
	MasonsTools,
	#[serde(rename = "painter's supplies")]
	PaintersSupplies,
	#[serde(rename = "potter's tools")]
	PottersTools,
	#[serde(rename = "smith's tools")]
	SmithsTools,
	#[serde(rename = "tinker's tools")]
	TinkersTools,
	#[serde(rename = "weaver's tools")]
	WeaversTools,
	#[serde(rename = "woodcarver's tools")]
	WoodcarversTools,
	#[serde(rename = "disguise kit")]
	DisguiseKit,
	#[serde(rename = "forgery kit")]
	ForgeryKit,
	#[serde(rename = "gaming set")]
	GamingSet,
	#[serde(rename = "dragonchess set")]
	DragonChess,
	#[serde(rename = "dice set")]
	DiceSet,
	#[serde(rename = "three-dragon ante set")]
	ThreeDragonAnteSet,
	#[serde(rename = "playing card set")]
	PlayerCadSet,
	#[serde(rename = "herbalism kit")]
	HerbalismKit,
	#[serde(rename = "musical instrument")]
	MusicalInstrument,
	#[serde(rename = "bagpipes")]
	BagPipe,
	#[serde(rename = "drum")]
	Drum,
	#[serde(rename = "dulcimer")]
	Dulcimer,
	#[serde(rename = "flute")]
	Flute,
	#[serde(rename = "horn")]
	Horn,
	#[serde(rename = "lute")]
	Lute,
	#[serde(rename = "lyre")]
	Lyre,
	#[serde(rename = "pan flute")]
	PanFlute,
	#[serde(rename = "shawm")]
	Shawm,
	#[serde(rename = "viol")]
	Viol,
	#[serde(rename = "navigator's tools")]
	NavigatorsTools,
	#[serde(rename = "thieves' tools")]
	ThievesTools,
	#[serde(rename = "poisoner's kit")]
	PoisonersKit,
	#[serde(rename = "vehicles")]
	Vehicles,
	#[serde(rename = "vehicles (air)")]
	VehiclesAir,
	#[serde(rename = "vehicles (land)")]
	VehiclesLand,
	#[serde(rename = "vehicles (water)")]
	VehiclesWater,
	#[serde(rename = "vehicles (space)")]
	VehiclesSpace,
	#[serde(rename = "athletics")]
	Athletics,
	#[serde(rename = "acrobatics")]
	Acrobatics,
	#[serde(rename = "sleight of hand")]
	SleightOfHand,
	#[serde(rename = "stealth")]
	Stealth,
	#[serde(rename = "arcana")]
	Arcana,
	#[serde(rename = "history")]
	History,
	#[serde(rename = "investigation")]
	Investigation,
	#[serde(rename = "nature")]
	Nature,
	#[serde(rename = "religion")]
	Religion,
	#[serde(rename = "animal handling")]
	AnimalHandling,
	#[serde(rename = "insight")]
	Insight,
	#[serde(rename = "medicine")]
	Medicine,
	#[serde(rename = "perception")]
	Perception,
	#[serde(rename = "survival")]
	Survival,
	#[serde(rename = "deception")]
	Deception,
	#[serde(rename = "intimidation")]
	Intimidation,
	#[serde(rename = "performance")]
	Performance,
	#[serde(rename = "persuasion")]
	Persuasion,
	#[serde(rename = "anySkill")]
	AnySkill,
	#[serde(rename = "anyTool")]
	AnyTool,
	#[serde(rename = "anyArtisansTool")]
	AnyArtisansTool,
	#[serde(rename = "anyMusicalInstrument")]
	AnyMusicalInstrument,
	#[serde(rename = "anyLanguage")]
	AnyLanguage,
	#[serde(rename = "anyStandardLanguage")]
	AnyStandardLanguage,
	#[serde(rename = "anyExoticLanguage")]
	AnyExoticLanguage,
	#[serde(rename = "anyRareLanguage")]
	AnyRareLanguage,
}

// TODO: shouldn't have default
#[derive(Debug, Default, Deserialize)]
pub enum SkillToolLanguageProficiencyChooseType {
	#[serde(rename = "weapon")]
	Weapon,
	#[serde(rename = "armor")]
	Armor,
	#[serde(rename = "skill")]
	Skill,
	#[serde(rename = "tool")]
	Tool,
	#[serde(rename = "language")]
	Language,
	#[serde(rename = "savingThrow")]
	SavingThrow,
	#[default]
	None,
}

// is a combination of two different possible objects
#[derive(Debug, Default, Deserialize)]
pub struct SkillToolLanguageProficiencyChoose {
	#[serde(default)]
	from: Vec<SkillToolLanguageProficiencyChooseFrom>,
	#[serde(default, rename = "fromFilter")]
	from_filter: String,
	#[serde(default)]
	count: i64,
	#[serde(default, rename = "type")]
	from_filter_type: SkillToolLanguageProficiencyChooseType,
}

#[derive(Debug, Default, Deserialize)]
pub struct SkillToolLanguageProficiency {
	#[serde(default, rename = "anyLanguage")]
	any_language: i64,
	#[serde(default, rename = "anyStandardLanguage")]
	any_standard_language: i64,
	#[serde(default, rename = "anyExoticLanguage")]
	any_exotic_language: i64,
	#[serde(default, rename = "anyRareLanguage")]
	any_rare_language: i64,
	#[serde(default)]
	abyssal: bool,
	#[serde(default)]
	celestial: bool,
	#[serde(default)]
	common: bool,
	#[serde(default, rename = "common sign language")]
	common_sign_language: bool,
	#[serde(default, rename = "deep speech")]
	deep_speech: bool,
	#[serde(default)]
	draconic: bool,
	#[serde(default)]
	infernal: bool,
	#[serde(default)]
	gith: bool,
	#[serde(default)]
	gnomish: bool,
	#[serde(default)]
	goblin: bool,
	#[serde(default)]
	halfling: bool,
	#[serde(default)]
	orc: bool,
	#[serde(default)]
	primordial: bool,
	#[serde(default)]
	aquan: bool,
	#[serde(default)]
	auran: bool,
	#[serde(default)]
	ignan: bool,
	#[serde(default)]
	terran: bool,
	#[serde(default)]
	sylvan: bool,
	#[serde(default)]
	undercommon: bool,
	#[serde(default)]
	dwarvish: bool,
	#[serde(default)]
	elvish: bool,
	#[serde(default)]
	giant: bool,
	#[serde(default)]
	other: bool,
	#[serde(default)]
	druidic: bool,
	#[serde(default, rename = "thieves' cant")]
	thieves_cant: bool,
	#[serde(default, rename = "otherLanguage")]
	other_language: bool,
	#[serde(default, rename = "anyTool")]
	any_tool: i64,
	#[serde(default, rename = "anyArtisansTool")]
	any_artisans_tool: i64,
	#[serde(default, rename = "artisan's tools")]
	artisans_tools: bool,
	#[serde(default, rename = "alchemist's supplies")]
	alchemists_supplies: bool,
	#[serde(default, rename = "brewer's supplies")]
	brewers_supplies: bool,
	#[serde(default, rename = "calligrapher's supplies")]
	calligraphers_supplies: bool,
	#[serde(default, rename = "carpenter's tools")]
	carpenters_tools: bool,
	#[serde(default, rename = "cartographer's tools")]
	cartographers_tools: bool,
	#[serde(default, rename = "cobbler's tools")]
	cobblers_tools: bool,
	#[serde(default, rename = "cook's utensils")]
	cooks_utensils: bool,
	#[serde(default, rename = "glassblower's tools")]
	glassblowers_tools: bool,
	#[serde(default, rename = "jeweler's tools")]
	jewelers_tools: bool,
	#[serde(default, rename = "leatherworker's tools")]
	leatherworkers_tools: bool,
	#[serde(default, rename = "mason's tools")]
	masons_tools: bool,
	#[serde(default, rename = "painter's supplies")]
	painters_supplies: bool,
	#[serde(default, rename = "potter's tools")]
	potters_tools: bool,
	#[serde(default, rename = "smith's tools")]
	smiths_tools: bool,
	#[serde(default, rename = "tinker's tools")]
	tinkers_tools: bool,
	#[serde(default, rename = "weaver's tools")]
	weavers_tools: bool,
	#[serde(default, rename = "woodcarver's tools")]
	woodcarvers_tools: bool,
	#[serde(default, rename = "disguise kit")]
	disguise_kit: bool,
	#[serde(default, rename = "forgery kit")]
	forgery_kit: bool,
	#[serde(default, rename = "anyGamingSet")]
	any_gaming_set: i64,
	#[serde(default, rename = "gaming set")]
	gaming_set: bool,
	#[serde(default, rename = "dragonchess set")]
	dragonchess_set: bool,
	#[serde(default, rename = "dice set")]
	dice_set: bool,
	#[serde(default, rename = "three-dragon ante set")]
	three_dragon_ante_set: bool,
	#[serde(default, rename = "playing card set")]
	playing_card_set: bool,
	#[serde(default, rename = "herbalism kit")]
	herbalism_kit: bool,
	#[serde(default, rename = "anyMusicalInstrument")]
	any_musical_instrument: i64,
	#[serde(default, rename = "musical instrument")]
	musical_instrument: bool,
	#[serde(default)]
	bagpipes: bool,
	#[serde(default)]
	drum: bool,
	#[serde(default)]
	dulcimer: bool,
	#[serde(default)]
	flute: bool,
	#[serde(default)]
	horn: bool,
	#[serde(default)]
	lute: bool,
	#[serde(default)]
	lyre: bool,
	#[serde(default, rename = "pan flute")]
	pan_flute: bool,
	#[serde(default)]
	shawm: bool,
	#[serde(default)]
	viol: bool,
	#[serde(default, rename = "navigator's tools")]
	navigators_tools: bool,
	#[serde(default, rename = "thieves' tools")]
	thieves_tools: bool,
	#[serde(default, rename = "poisoner's kit")]
	poisoners_kit: bool,
	#[serde(default)]
	vehicles: bool,
	#[serde(default, rename = "vehicles (air)")]
	vehicles_air: bool,
	#[serde(default, rename = "vehicles (land)")]
	vehicles_land: bool,
	#[serde(default, rename = "vehicles (water)")]
	vehicles_water: bool,
	#[serde(default, rename = "vehicles (space)")]
	vehicles_space: bool,
	#[serde(default, rename = "anySkill")]
	any_skill: i64,
	#[serde(default)]
	athletics: bool,
	#[serde(default)]
	acrobatics: bool,
	#[serde(default, rename = "sleight of hand")]
	sleight_of_hand: bool,
	#[serde(default)]
	stealth: bool,
	#[serde(default)]
	arcana: bool,
	#[serde(default)]
	history: bool,
	#[serde(default)]
	investigation: bool,
	#[serde(default)]
	nature: bool,
	#[serde(default)]
	religion: bool,
	#[serde(default, rename = "animal handling")]
	animal_handling: bool,
	#[serde(default)]
	insight: bool,
	#[serde(default)]
	medicine: bool,
	#[serde(default)]
	perception: bool,
	#[serde(default)]
	survival: bool,
	#[serde(default)]
	deception: bool,
	#[serde(default)]
	intimidation: bool,
	#[serde(default)]
	performance: bool,
	#[serde(default)]
	persuasion: bool,
	#[serde(default)]
	choose: Vec<SkillToolLanguageProficiencyChoose>,
}

pub type SkillToolLanguageProficiencies = Vec<SkillToolLanguageProficiency>;

#[derive(Debug, Default, Deserialize)]
pub struct WeaponProficiencyChoose {
	#[serde(default, rename = "fromFilter")]
	// A filter string, e.g. "type=martial weapon|miscellaneous=mundane"
	from_filter: String,
	#[serde(default)]
	count: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct WeaponProficiencyAll {
	#[serde(default, rename = "fromFilter")]
	// A filter string, e.g. "type=martial weapon|miscellaneous=mundane"
	from_filter: String,
}

/**
 * TODO: deal with dynamic properties
 * "^[^|]+\\|.+$": {
 *     "const": true
 * }
 */
#[derive(Debug, Default, Deserialize)]
pub struct WeaponProficiency {
	#[serde(default)]
	simple: bool,
	#[serde(default)]
	martial: bool,
	#[serde(default)]
	firearms: bool,
	#[serde(default)]
	improvised: bool,
	#[serde(default)]
	choose: WeaponProficiencyChoose,
	#[serde(default)]
	all: WeaponProficiencyAll,
}

pub type WeaponProficiencies = Vec<WeaponProficiency>;

#[derive(Debug, Default, Deserialize)]
pub struct ArmorProficiency {
	#[serde(default)]
	light: bool,
	#[serde(default)]
	medium: bool,
	#[serde(default)]
	heavy: bool,
	#[serde(default)]
	shield: bool,
}

pub type ArmorProficiencies = Vec<ArmorProficiency>;

#[derive(Debug, Default, Deserialize)]
pub struct AdditionalFeatsArrayItemCategory {
	#[serde(default)]
	category: Vec<DataFeatCategory>,
	#[serde(default)]
	count: i64,
}

/**
 * TODO: handle dynamic fields
 * "^[^|]+\\|.+$": {
 *     "const": true
 * }
 */
#[derive(Debug, Default, Deserialize)]
pub struct AdditionalFeatsArrayItem {
	#[serde(default)]
	any: i64,
	#[serde(default, rename = "anyFromCategory")]
	any_from_category: AdditionalFeatsArrayItemCategory,
}

pub type AdditionalFeatsArray = Vec<AdditionalFeatsArrayItem>;

/**
 * D: Dragonmark
 * G: General
 * O: Origin
 * FS: Fighting Style
 * FS:P: Fighting Style (Paladin)
 * FS:R: Fighting Style (Ranger)
 * EB: Epic Boon
 */
#[derive(Debug, Deserialize)]
pub enum DataFeatCategory {
	D,
	G,
	O,
	FS,
	#[serde(rename = "FS:P")]
	FSP,
	#[serde(rename = "FS:R")]
	FSR,
	EB,
}

pub type SRD = String;

pub fn default_srd_value() -> String {
	String::from("false")
}

pub fn deserialize_srd<'de, D>(deserializer: D) -> Result<SRD, D::Error>
where
	D: Deserializer<'de>,
{
	struct SRDVisitor;

	impl<'de> Visitor<'de> for SRDVisitor {
		type Value = SRD;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			Ok(v.to_string() as SRD)
		}
	}

	return deserializer.deserialize_any(SRDVisitor);
}

pub type BasicRules = bool;
