use serde::Deserialize;

use crate::dao::utils::{Alias, BasicRules, Page, ReprintedAs, SRD, Source, default_srd_value, deserialize_srd};

#[derive(Debug, Default, Deserialize)]
struct EntryDataDataGenTables {
	#[serde(default, rename = "tableIgnore")]
	table_ignore: bool,
	#[serde(default, rename = "tableChapterIgnore")]
	table_chapter_ignore: bool,
	#[serde(default, rename = "tableInclude")]
	table_include: bool,
	#[serde(default, rename = "tableNamePrefix")]
	table_name_prefix: String,
	#[serde(default, rename = "tableName")]
	table_name: String,
	#[serde(default, rename = "skipSectionPrefix")]
	skip_section_prefix: bool,
	#[serde(default, rename = "reprintedAs")]
	reprinted_as: ReprintedAs,
	#[serde(default, rename = "fauxGroupName")]
	faux_group_name: String,
	#[serde(default, rename = "fauxGroupSource")]
	faux_group_source: Source,
	#[serde(default, rename = "fauxGroupPage")]
	faux_group_page: Page,
	#[serde(default, rename = "isFauxGroupAdditional")]
	is_faux_group_additional: bool,
}

/**
 * A generic object for storing special data for external use-cases.
 * Keys prefixed with "rd-" should be added as "data-" HTML attributes when rendering to HTML.
 */
#[derive(Debug, Default, Deserialize)]
struct EntryDataData {
	#[serde(default, rename = "genTables")]
	gen_tables: EntryDataDataGenTables,
}

// TODO: maybe shouldn't have default
#[derive(Debug, Default, Deserialize)]
pub enum EntryType {
	#[serde(rename = "section")]
	Section,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntrySection {
	#[serde(default)]
	name: String,
	#[serde(default, rename = "type")]
	entry_type: EntryType,
	#[serde(default)]
	source: Source,
	#[serde(default)]
	page: Page,
	#[serde(default)]
	data: EntryDataData,
	#[serde(default)]
	id: String,
	#[serde(default = "default_srd_value", deserialize_with = "deserialize_srd")]
	srd: SRD,
	#[serde(default = "default_srd_value", deserialize_with = "deserialize_srd")]
	srd52: SRD,
	#[serde(default, rename = "basicRules")]
	basic_rules: BasicRules,
	#[serde(default, rename = "basicRules2024")]
	basic_rules_2024: BasicRules,
	#[serde(default)]
	alias: Alias,
	#[serde(default)]
	entries: Vec<Entry>,
	#[serde(default)]
	style: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryEntries {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryQuote {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryInlineEntries {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryEntriesInlineEntries {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryOptions {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryTableGroup {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryTable {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryTableRow {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryTableCell {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryList {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryBonus {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryBonusSpeed {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryDice {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryAbilityDc {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryAbilityAttackMod {}

#[derive(Debug, Default, Deserialize)]
pub struct AbilityGeneric {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryLink {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryOptFeature {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryInset {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryInsetReadaloud {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryVariant {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryVariantInner {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryVariantSub {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryItem {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryItemSub {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryItemSpell {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryImage {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryGallery {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryActions {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryAttack {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryStatblockInline {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryStatblock {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryRefClassFeature {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryRefSubclassFeature {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryRefOptionalfeature {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryRefFeat {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryHr {}

#[derive(Debug, Default, Deserialize)]
pub struct EntrySpellcasting {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryFlowchart {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryFlowBlock {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryIngredient {}

#[derive(Debug, Default, Deserialize)]
pub struct EntryWrapped {}

// An entry can be any one of the above types including either a string or an integer.

#[derive(Debug, Default, Deserialize)]
pub struct Entry {}
