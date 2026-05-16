use serde::{
	Deserialize, Deserializer,
	de::{self, Visitor},
};

use crate::dao::{
	common::ItemType,
	items_shared::{ItemPropertyArray, deserialize_item_property, deserialize_item_type},
	utils_edition::Edition,
};

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

#[derive(Debug)]
enum PrerequisiteSpellcastingFocusTool {
	Arcane,
	Druid,
	Holy,
	Tool,
	ArtisansTool,
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

		fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			Ok(PrerequisiteSpellcastingFocus { needs_focus: true, focus_tools: vec![] })
		}

		fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
		where
			A: de::SeqAccess<'de>,
		{
			let focus_tools: Vec<PrerequisiteSpellcastingFocusTool> = vec![];
			for focus_tool in seq {
				focus_tools.push(match focus_tool {
					"arcane" => PrerequisiteSpellcastingFocusTool::Arcane,
					"druid" => PrerequisiteSpellcastingFocusTool::Druid,
					"holy" => PrerequisiteSpellcastingFocusTool::Holy,
					"tool" => PrerequisiteSpellcastingFocusTool::Tool,
					"artisansTool" => PrerequisiteSpellcastingFocusTool::ArtisansTool,
					_ => return Err(E::custom("had an invalid spellcasting focus tool")),
				});
			}
			Ok(PrerequisiteSpellcastingFocus { needs_focus: true, focus_tools: focus_tools })
		}
	}

	return deserializer.deserialize_string(ReprintedAsVisitor);
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
	pact: PrerequisitePact,
	patron: PrerequisitePatron,
	spell: PrerequisiteSpell,
	feat: Vec<String>,
	feat_category: Vec<DataFeatCategory>,
	exclusive_feat_category: Vec<DataFeatCategory>,
	#[serde(rename = "optionalfeature")]
	optional_feature: Vec<String>,
	feature: Vec<String>,
	item: Vec<String>,
	#[serde(deserialize_with = "deserialize_item_type")]
	item_type: Vec<ItemType>,
	#[serde(deserialize_with = "deserialize_item_property")]
	item_property: ItemPropertyArray,
	/** A free text prerequisite */
	other: String,
	/** A free text prerequisite, with a shortened form for list display. */
	other_summary: OtherSummary,
	race: Vec<PrerequisiteRace>,
	background: Vec<PrerequisiteBackground>,
	ability: Vec<PrerequisiteAbility>,
	spellcasting: bool,
	/** Renders with the updated text found in UA2020: Feats */
	spellcasting2020: bool,
	/** A more restrictive spellcasting variant; specifically \"Spellcasting Feature\" */
	spellcasting_feature: bool,
	/** A more restrictive spellcasting variant; specifically \"Spellcasting feature from a class that prepares spells\" */
	spellcasting_prepared: bool,
	psionics: bool,
	#[serde(deserialize_with = "deserialize_prerequisite_spellcasting_focus")]
	spellcasting_focus: PrerequisiteSpellcastingFocus,
	proficiency: Vec<PrerequisiteProficiency>,
	expertise: Vec<PrerequisteExpertise>,
	alignment: Vec<Alignment>,
	campaign: Vec<PrerequisiteCampaign>,
	/** Example: "Elven" */
	culture: Vec<String>,
	#[serde(deserialize_with = "deserialize_prerequisite_membership")]
	membership: Vec<PrerequisiteMembership>,
	group: Vec<PrerequisiteGroup>,
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
	A
}

#[derive(Debug, Default, Deserialize)]
pub struct DataFeatCategory {}
