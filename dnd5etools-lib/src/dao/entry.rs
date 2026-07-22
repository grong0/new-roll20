use serde::{
	Deserialize, Deserializer,
	de::{self, Visitor},
};

use crate::dao::{
	bestiary::Creature,
	utils::{
		AbilityScoreAbbreviation, Alias, BasicRules, MetaDependenciesArray, Page, ReprintedAs, SRD, Source, TagNameStats,
		default_srd_value, deserialize_srd,
	},
};

#[derive(Debug, Default, Deserialize)]
pub struct Spell {
	#[serde(default)]
	entry: String,
	#[serde(default)]
	hidden: bool,
}

pub type ArrayOfSpell = Vec<Spell>;

fn deserialize_array_of_spells<'de, D>(deserializer: D) -> Result<ArrayOfSpell, D::Error>
where
	D: Deserializer<'de>,
{
	struct ArrayOfSpellVisitor;

	impl<'de> Visitor<'de> for ArrayOfSpellVisitor {
		type Value = ArrayOfSpell;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}
	}

	return deserializer.deserialize_any(ArrayOfSpellVisitor);
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum EntryDataStyle {
	#[serde(rename = "inset")]
	Inset,
	#[serde(rename = "narrow")]
	Narrow,
	#[default]
	None,
}

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
pub struct EntryDataData {
	#[serde(default, rename = "genTables")]
	gen_tables: EntryDataDataGenTables,
}

// TODO: handle case where it is a string
pub type EntryTableColLabelItem = EntryTableHeaderCell;

#[derive(Debug, Default, Deserialize)]
pub struct MediaHrefInternal {
	// always "internal"
	#[serde(default, rename = "type")]
	href_type: String,
	// pattern: "\\.(webp|mp3)$"
	#[serde(default)]
	path: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct MediaHrefExternal {
	// always "external"
	#[serde(default, rename = "type")]
	href_type: String,
	/**
	 * Restrict the usage of:
	 * - Site URLs -- these should be `"internal"` URLs instead
	 * - Imgur links -- these are not accessible in the UK https://help.imgur.com/hc/en-us/articles/41592665292443-Imgur-access-in-the-United-Kingdom
	 *
	 * "pattern": "^https?://",
	 * "not": {
	 * 	"pattern": "^https?://(5e\\.tools|5etools-mirror-\\d+\\.github\\.io|raw\\.githubusercontent\\.com/5etools-mirror-\\d+|i\\.imgur\\.com|imgur\\.com)/"
	 * }
	 */
	#[serde(default)]
	url: String,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum MediaHrefType {
	#[serde(rename = "internal")]
	Internal,
	#[serde(rename = "external")]
	External,
	#[default]
	None,
}

/**
 * Combination of MediaHrefInternal and MediaHrefExternal
 */
#[derive(Debug, Default, Deserialize)]
pub struct MediaHref {
	#[serde(default, rename = "type")]
	href_type: MediaHrefType,
	// pattern: "\\.(webp|mp3)$"
	#[serde(default)]
	path: String,
	/**
	 * Restrict the usage of:
	 * - Site URLs -- these should be `"internal"` URLs instead
	 * - Imgur links -- these are not accessible in the UK https://help.imgur.com/hc/en-us/articles/41592665292443-Imgur-access-in-the-United-Kingdom
	 *
	 * "pattern": "^https?://",
	 * "not": {
	 * 	"pattern": "^https?://(5e\\.tools|5etools-mirror-\\d+\\.github\\.io|raw\\.githubusercontent\\.com/5etools-mirror-\\d+|i\\.imgur\\.com|imgur\\.com)/"
	 * }
	 */
	#[serde(default)]
	url: String,
}

// // TODO: resolve dual type
// pub type MediaHref = MediaHrefExternal | MediaHrefInternal;

#[derive(Debug, Default, Deserialize)]
pub struct EntryEntry {
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
}

// TODO: maybe shouldn't have default
#[derive(Debug, Default, Deserialize)]
pub enum EntryType {
	#[serde(rename = "section")]
	Section,
	#[serde(rename = "entries")]
	Entries,
	#[serde(rename = "quote")]
	Quote,
	#[serde(rename = "inline")]
	Inline,
	#[serde(rename = "inlineBlock")]
	InlineBlock,
	#[serde(rename = "options")]
	Options,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntrySection {
	#[serde(default)]
	name: String,
	// Always "section"
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
pub struct EntryEntries {
	#[serde(default)]
	name: String,
	// Always "entries"
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
	entries: Vec<Entry>,
	#[serde(default)]
	alias: Alias,
	#[serde(default)]
	style: String,
}

type EntryQuoteBy = Vec<String>;

fn deserialize_entry_quote_by<'de, D>(deserializer: D) -> Result<EntryQuoteBy, D::Error>
where
	D: Deserializer<'de>,
{
	struct EntryQuoteByVisitor;

	impl<'de> Visitor<'de> for EntryQuoteByVisitor {
		type Value = EntryQuoteBy;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("A UID, e.g. \"longsword|phb\", or a ReprintedAs object");
		}

		fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			Ok(vec![v])
		}
	}

	return deserializer.deserialize_any(EntryQuoteByVisitor);
}

#[derive(Debug, Default, Deserialize)]
pub enum EntryQuoteStyle {
	#[serde(rename = "quote-pull")]
	QuotePull,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryQuote {
	#[serde(default)]
	name: String,
	// Always "quote"
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
	entries: Vec<Entry>,
	#[serde(default, deserialize_with = "deserialize_entry_quote_by")]
	by: EntryQuoteBy,
	#[serde(default)]
	from: String,
	// If the automatically-inserted quotation marks should be skipped.
	#[serde(default, rename = "skipMarks")]
	skip_marks: bool,
	#[serde(default, rename = "skipItalics")]
	skip_italics: bool,
	#[serde(default)]
	style: EntryQuoteStyle,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryInlineEntries {
	#[serde(default)]
	name: String,
	// Always "inline"
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
	entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryEntriesInlineEntries {
	#[serde(default)]
	name: String,
	// Always "inlineBlock"
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
	entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryOptions {
	#[serde(default)]
	name: String,
	// Always "options"
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
	// Used to specify how many of the listed options can be chosen as e.g. permanent character features. Leave blank for transient choices.
	#[serde(default)]
	count: i64,
	#[serde(default)]
	style: String,
	#[serde(default)]
	entries: Vec<Entry>,
}

// Used to group related tables together; has no effect on rendering.
#[derive(Debug, Default, Deserialize)]
pub struct EntryTableGroup {
	#[serde(default)]
	name: String,
	// Always "tableGroup"
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
	#[serde(default)]
	tables: Vec<EntryTable>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryTable {
	#[serde(default)]
	name: String,
	// Always "table"
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
	#[serde(default)]
	srd: SRD,
	#[serde(default)]
	srd52: SRD,
	#[serde(default, rename = "basicRules")]
	basic_rules: BasicRules,
	#[serde(default, rename = "basicRules2024")]
	basic_rules_2024: BasicRules,
	#[serde(default)]
	caption: String,
	#[serde(default, rename = "isStriped")]
	is_striped: bool,
	// Uses the result rolled to construct a name from multiple columns, such as Prefix, Given, Family, Suffix names etc.
	#[serde(default, rename = "isNameGenerator")]
	is_name_generator: bool,
	#[serde(default)]
	style: String,
	#[serde(default, rename = "colLabels")]
	col_labels: Vec<EntryTableColLabelItem>,
	#[serde(default, rename = "colLabelRows")]
	col_label_rows: Vec<Vec<EntryTableColLabelItem>>,
	#[serde(default, rename = "colStyles")]
	col_styles: Vec<String>,
	#[serde(default, rename = "rowLabels")]
	row_labels: Vec<String>,
	#[serde(default, rename = "rowStyles")]
	row_styles: Vec<String>,
	// TODO: resolve dual type
	// #[serde(default)]
	// rows: Vec<Vec<Entry> | EntryTableRow>,
	#[serde(default)]
	footnotes: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryTableRow {
	#[serde(default)]
	name: String,
	// Always "row"
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
	#[serde(default)]
	style: String,
	#[serde(default)]
	row: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
pub enum EntryTableHeaderCellStyle {
	#[default]
	#[serde(rename = "ThSkewer")]
	ThSkewer,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryTableHeaderCell {
	#[serde(default)]
	name: String,
	// Always "row"
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
	#[serde(default)]
	srd: SRD,
	#[serde(default)]
	srd52: SRD,
	#[serde(default, rename = "basicRules")]
	basic_rules: BasicRules,
	#[serde(default, rename = "basicRules2024")]
	basic_rules_2024: BasicRules,
	// minimum: 2
	#[serde(default)]
	width: i64,
	#[serde(default)]
	entry: String,
	#[serde(default)]
	style: EntryTableHeaderCellStyle,
}

#[derive(Debug, Default, Deserialize)]
struct EntryTableCellRollMinMax {
	#[serde(default)]
	min: f64,
	#[serde(default)]
	max: f64,
	#[serde(default)]
	pad: bool,
}

#[derive(Debug, Default, Deserialize)]
struct EntryTableCellRollExact {
	#[serde(default)]
	exact: f64,
	#[serde(default)]
	pad: bool,
}

/**
 * Combination of EntryTableCellRollMinMax and EntryTableCellRollExact
 */
#[derive(Debug, Default, Deserialize)]
pub struct EntryTableCellRoll {
	#[serde(default)]
	min: i64,
	#[serde(default)]
	max: i64,
	#[serde(default)]
	exact: i64,
	#[serde(default)]
	pad: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryTableCell {
	#[serde(default)]
	name: String,
	// Always "cell"
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
	width: i64,
	roll: EntryTableCellRoll,
	entry: Entry,
}

/**
 * TODO: maybe shouldn't have default
 * Formatting to be applied to the list; not providing a style results in a simple bullet point list.
 * - list-decimal: decimal numbering 1. 2. 3. etc
 * - list-hang: a list with the bullets removed and a full indentation
 * - list-hang-notitle: a list with no bullets that indents all lines but the first
 * - list-lower-roman: lowercase roman numeral numbering i, ii, iii, iv, etc.
 * - list-name: a bulleted list with all text in bold
 * - list-no-bullets: a list with the bullets removed and reduced indentation
*/
#[derive(Debug, Default, Deserialize)]
pub enum EntryListStyle {
	#[serde(rename = "list-decimal")]
	ListDeciaml,
	#[serde(rename = "list-hang")]
	ListHand,
	#[serde(rename = "list-hang-notitle")]
	ListHandNoTitle,
	#[serde(rename = "list-hang-subtrait")]
	ListHandSubtrait,
	#[serde(rename = "list-lower-roman")]
	ListLowerRoman,
	#[serde(rename = "list-upper-roman")]
	ListUpperRoman,
	#[serde(rename = "list-name")]
	ListName,
	#[serde(rename = "list-no-bullets")]
	ListNoBullets,
	#[serde(rename = "list-caption")]
	ListCaption,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryList {
	#[serde(default)]
	name: String,
	// Always "list"
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
	// umber of columns the content should be split into. Note that the full value is only displayed on wide screens, and screens below certain widths will see an appropriately reduced number of columns.
	#[serde(default)]
	columns: i64,
	#[serde(default)]
	style: EntryListStyle,
	#[serde(default)]
	start: i64,
	#[serde(default)]
	items: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryBonus {
	#[serde(default)]
	name: String,
	// Always "bonus"
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
	#[serde(default)]
	value: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryBonusSpeed {
	#[serde(default)]
	name: String,
	// Always "bonusSpeed"
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
	#[serde(default)]
	value: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryDiceToRoll {
	#[serde(default)]
	number: i64,
	#[serde(default)]
	faces: i64,
	// (Warning: unused)
	#[serde(default)]
	modifier: i64,
	// (Warning: unused)
	#[serde(default, rename = "hideModifier")]
	hide_modifier: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryDice {
	#[serde(default)]
	name: String,
	// Always "dice"
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
	#[serde(default)]
	value: f64,
	#[serde(default, rename = "toRoll")]
	to_roll: Vec<EntryDiceToRoll>,
	#[serde(default)]
	rollable: bool,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum EntryAbilityAttributes {
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
	#[serde(rename = "spellcasting")]
	Spellcasting,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryAbilityDc {
	#[serde(default)]
	name: String,
	// Always "abilityDc"
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
	#[serde(default)]
	attributes: Vec<EntryAbilityAttributes>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryAbilityAttackMod {
	#[serde(default)]
	name: String,
	// Always "abilityAttackMod"
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
	#[serde(default)]
	attributes: Vec<EntryAbilityAttributes>,
}

#[derive(Debug, Default, Deserialize)]
pub struct AbilityGeneric {
	#[serde(default)]
	name: String,
	// Always "abilityAttackMod"
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
	#[serde(default)]
	text: String,
	#[serde(default)]
	attributes: Vec<AbilityScoreAbbreviation>,
}

pub fn deserialize_entry_link_href_subhash_values<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
	D: Deserializer<'de>,
{
	struct EntryLinkHrefSubhashValuesVisitor;

	impl<'de> Visitor<'de> for EntryLinkHrefSubhashValuesVisitor {
		type Value = Vec<String>;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			Ok(vec![v.to_string()])
		}
	}

	return deserializer.deserialize_any(EntryLinkHrefSubhashValuesVisitor);
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryLinkHrefSubhash {
	#[serde(default)]
	key: String,
	#[serde(default, alias = "value", deserialize_with = "deserialize_entry_link_href_subhash_values")]
	values: Vec<String>,
	#[serde(default, rename = "preEncoded")]
	pre_encoded: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryLinkHrefHover {
	#[serde(default)]
	page: Page,
	#[serde(default)]
	source: Source,
	#[serde(default)]
	hash: String,
	#[serde(default, rename = "hashPreEncoded")]
	hash_pre_encoded: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryLinkHref {
	// always "internal"
	#[serde(default)]
	href_type: String,
	#[serde(default)]
	path: String,
	#[serde(default)]
	hash: String,
	#[serde(default, rename = "hashPreEncoded")]
	hash_pre_encoded: bool,
	#[serde(default)]
	subhashes: Vec<EntryLinkHrefSubhash>,
	#[serde(default)]
	hover: EntryLinkHrefHover,
}

/**
 * Is EntryLinkHref with the addition of a url field to support MediaHrefExternal objects.
 */
#[derive(Debug, Default, Deserialize)]
pub struct EntryLinkHrefCombination {
	#[serde(default)]
	href_type: String,
	#[serde(default)]
	path: String,
	#[serde(default)]
	url: String,
	#[serde(default)]
	hash: String,
	#[serde(default, rename = "hashPreEncoded")]
	hash_pre_encoded: bool,
	#[serde(default)]
	subhashes: Vec<EntryLinkHrefSubhash>,
	#[serde(default)]
	hover: EntryLinkHrefHover,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryLink {
	#[serde(default)]
	name: String,
	// Always "abilityAttackMod"
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
	#[serde(default)]
	text: String,
	#[serde(default)]
	href: EntryLinkHrefCombination,
}

/**
 * For e.g. Eldritch Invocations which require prerequisite text.
 * Deprecated; prefer "refOptionalfeature" or "statblock" entries instead.
 */
#[derive(Debug, Default, Deserialize)]
pub struct EntryOptFeature {
	#[serde(default)]
	name: String,
	// Always "optfeature"
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
	#[serde(default)]
	deprecated: bool,
	#[serde(default)]
	prerequisite: String,
	#[serde(default)]
	entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryInset {
	#[serde(default)]
	name: String,
	// Always "abilityAttackMod"
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
	#[serde(default)]
	entries: Vec<Entry>,
	#[serde(default)]
	style: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryInsetReadaloud {
	#[serde(default)]
	name: String,
	// Always "insetReadaloud"
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
	#[serde(default)]
	entries: Vec<Entry>,
	#[serde(default)]
	style: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryVariant {
	#[serde(default)]
	name: String,
	// Always "variant"
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
	#[serde(default)]
	entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryVariantInner {
	#[serde(default)]
	name: String,
	// Always "variantInner"
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
	#[serde(default)]
	entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryVariantSub {
	#[serde(default)]
	name: String,
	// Always "variantSub"
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
	#[serde(default)]
	entries: Vec<Entry>,
}

pub fn deserialize_entry_item_entries<'de, D>(deserializer: D) -> Result<Vec<Entry>, D::Error>
where
	D: Deserializer<'de>,
{
	struct EntryItemEntriesVisitor;

	impl<'de> Visitor<'de> for EntryItemEntriesVisitor {
		type Value = Vec<Entry>;

		fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
			return formatter.write_str("");
		}
	}

	return deserializer.deserialize_any(EntryItemEntriesVisitor);
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryItem {
	#[serde(default)]
	name: String,
	// Always "item"
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
	#[serde(default)]
	style: String,
	// might not work. couldn't figure out how to keep normal deserialze map function, but return a vector
	#[serde(default, alias = "entry")]
	entries: Vec<Entry>,
	#[serde(default, rename = "nameDot")]
	name_dot: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryItemSub {
	#[serde(default)]
	name: String,
	// Always "itemSub"
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
	// might not work. couldn't figure out how to keep normal deserialze map function, but return a vector
	#[serde(default, alias = "entry")]
	entries: Vec<Entry>,
	#[serde(default, rename = "nameDot")]
	name_dot: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryItemSpell {
	#[serde(default)]
	name: String,
	// Always "itemSpell"
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
	#[serde(default)]
	entry: Entry,
}

#[derive(Debug, Default, Deserialize)]
pub enum EntryImageImageType {
	#[serde(rename = "map")]
	Map,
	#[serde(rename = "mapPlayer")]
	MapPlayer,
	#[default]
	None,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum EntryImageGridType {
	#[default]
	#[serde(rename = "none")]
	None,
	#[serde(rename = "square")]
	Square,
	#[serde(rename = "hexRowsOdd")]
	HexRowsOdd,
	#[serde(rename = "hexRowsEven")]
	HexRowsEven,
	#[serde(rename = "hexColsOdd")]
	HexColsOdd,
	#[serde(rename = "hexColsEven")]
	HexColsEven,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum EntryImageGridUnits {
	#[serde(rename = "feet")]
	Feet,
	#[serde(rename = "yards")]
	Yards,
	#[serde(rename = "miles")]
	Miles,
	#[serde(rename = "meters")]
	Meters,
	#[serde(rename = "kilometers")]
	Kilometers,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryImageGrid {
	#[serde(default, rename = "type")]
	grid_type: EntryImageGridType,
	// The size of a single grid square/hex, in pixels.
	#[serde(default)]
	size: i64,
	#[serde(default, rename = "offsetX")]
	offset_x: i64,
	#[serde(default, rename = "offsetY")]
	offset_y: i64,
	// Map image scaling. Where possible, avoid using this, and use size/offset instead.
	#[serde(default)]
	scale: f64,
	// A unit of measurement (e.g. "feet", "miles") in which the grid should be presented.
	#[serde(default)]
	units: EntryImageGridUnits,
	#[serde(default)]
	distance: f64,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryImageMapRegion {
	#[serde(default)]
	area: String,
	// max items and min items for inside vector is 2
	#[serde(default)]
	points: Vec<Vec<i64>>,
}

/**
 * Note that the order of transformations is: scale, then offset.
 */
#[derive(Debug, Default, Deserialize)]
pub struct EntryImageMapParent {
	#[serde(default)]
	id: String,
	// The X position of this map within the parent map. Defaults to 0.
	#[serde(default, rename = "offsetX")]
	offset_x: i64,
	// The Y position of this map within the parent map. Defaults to 0.
	#[serde(default, rename = "offsetY")]
	offset_y: i64,
	// X scaling required for this map to match the parent map. Defaults to 1.
	#[serde(default, rename = "scaleX")]
	scale_x: f64,
	// Y scaling required for this map to match the parent map. Defaults to 1.
	#[serde(default, rename = "scaleY")]
	scale_y: f64,
	// If the X/Y scaling for this map should be based on the size ratio between the two. This is often used a placeholder for maps requiring future manual adjustment.
	#[serde(default, rename = "autoScale")]
	auto_scale: bool,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum EntryImageStyle {
	#[serde(rename = "comic-speaker-left")]
	ComicSpeakerLeft,
	#[serde(rename = "comic-speaker-right")]
	ComicSpeakerRight,
	#[serde(rename = "deity-symbol")]
	DeitySymbol,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryImage {
	#[serde(default)]
	name: String,
	// Always "image"
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
	#[serde(default)]
	href: MediaHref,
	// A thumbnail image used in rare cases, e.g. when loading a wall of maps to choose from in the DM Screen.
	#[serde(default)]
	href_thumbnail: MediaHref,
	#[serde(default)]
	title: String,
	#[serde(default)]
	credit: String,
	#[serde(default, rename = "altText")]
	alt_text: String,
	#[serde(default, rename = "imageType")]
	image_type: EntryImageImageType,
	#[serde(default)]
	grid: EntryImageGrid,
	#[serde(default, rename = "mapRegions")]
	map_regions: Vec<EntryImageMapRegion>,
	#[serde(default, rename = "labelMapRegions")]
	label_map_regions: bool,
	// Note that the order of transformations is: scale, then offset.
	#[serde(default, rename = "mapParent")]
	map_parent: EntryImageMapParent,
	#[serde(default, rename = "mapName")]
	map_name: String,
	#[serde(default)]
	width: i64,
	#[serde(default)]
	height: i64,
	// Specify the max desired display width of the images, as opposed to "width" which should only be used for the _real_ width. Assumes pixels by default.
	#[serde(default, rename = "maxWidth")]
	max_width: i64,
	// As per "maxWidth"
	#[serde(default, rename = "maxHeight")]
	max_height: i64,
	#[serde(default, rename = "maxWidthUnits")]
	max_width_units: String,
	#[serde(default, rename = "maxHeightUnits")]
	max_height_units: String,
	#[serde(default)]
	style: EntryImageStyle,
	// If this image is largely transparent, and expects to be rendered atop a light background.
	#[serde(default, rename = "expectsLightBackground")]
	expects_light_background: bool,
	// If this image is largely transparent, and expects to be rendered atop a dark background.
	#[serde(default, rename = "expectsDarkBackground")]
	expects_dark_background: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryGallery {
	#[serde(default)]
	name: String,
	// Always "gallery"
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
	#[serde(default)]
	images: Vec<EntryImage>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryActions {
	#[serde(default)]
	name: String,
	// Always "actions"
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
	#[serde(default)]
	entries: Vec<Entry>,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum EntryAttackType {
	MW,
	RW,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryAttack {
	#[serde(default)]
	name: String,
	// Always "attack"
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
	#[serde(default, rename = "attackType")]
	attack_type: EntryAttackType,
	#[serde(default, rename = "attackEntries")]
	attack_entries: Vec<Entry>,
	#[serde(default, rename = "hitEntries")]
	hit_entries: Vec<Entry>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryStatblockInline {
	// Always "statblockInline"
	#[serde(default, rename = "type")]
	entry_type: EntryType,
	#[serde(default)]
	depdendencies: MetaDependenciesArray,
	// Always "monster"
	#[serde(default)]
	data_type: String,
	#[serde(default)]
	data: Creature,
	#[serde(default)]
	style: EntryDataStyle,
	#[serde(default)]
	collapsed: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryStatblock {
	#[serde(default)]
	name: String,
	// Always "attack"
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
	#[serde(default, rename = "displayName")]
	display_name: String,
	#[serde(default)]
	hash: String,
	#[serde(default)]
	style: EntryDataStyle,
	#[serde(default)]
	collapsed: bool,
	#[serde(default)]
	tag: TagNameStats,
}

/**
 * For use in classes page content only.
 */
#[derive(Debug, Default, Deserialize)]
pub struct EntryRefClassFeature {
	// Always "refClassFeature"
	#[serde(default, rename = "type")]
	entry_type: String,
	#[serde(default, rename = "classFeature")]
	class_feature: String,
	#[serde(default)]
	data: EntryDataData,
}

/**
 * For use in classes page content only.
 */
#[derive(Debug, Default, Deserialize)]
pub struct EntryRefSubclassFeature {
	// Always "refSubclassFeature"
	#[serde(default, rename = "type")]
	entry_type: String,
	#[serde(default, rename = "subclassFeature")]
	subclass_feature: String,
	data: EntryDataData,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryRefOptionalFeaturePreserve {
	#[serde(default)]
	prerequisite: bool,
	#[serde(default)]
	consumes: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryRefOptionalFeature {
	// Always "refOptionalfeature"
	#[serde(default, rename = "type")]
	entry_type: String,
	#[serde(default, rename = "refOptionalfeature")]
	optional_feature: String,
	#[serde(default)]
	name: String,
	#[serde(default)]
	preserve: EntryRefOptionalFeaturePreserve,
	#[serde(default)]
	data: EntryDataData,
}

/**
 * For use in classes page content only.
 */
#[derive(Debug, Default, Deserialize)]
pub struct EntryRefFeat {
	// Always "refFeat"
	#[serde(default, rename = "type")]
	entry_type: String,
	#[serde(default)]
	feat: String,
	#[serde(default)]
	name: String,
	#[serde(default)]
	data: EntryDataData,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryHr {
	// Always "hr"
	#[serde(default, rename = "type")]
	entry_type: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntrySpellcastingFrequency {
	#[serde(default, rename = "1")]
	level_1: ArrayOfSpell,
	#[serde(default, rename = "2")]
	level_2: ArrayOfSpell,
	#[serde(default, rename = "3")]
	level_3: ArrayOfSpell,
	#[serde(default, rename = "4")]
	level_4: ArrayOfSpell,
	#[serde(default, rename = "5")]
	level_5: ArrayOfSpell,
	#[serde(default, rename = "6")]
	level_6: ArrayOfSpell,
	#[serde(default, rename = "7")]
	level_7: ArrayOfSpell,
	#[serde(default, rename = "8")]
	level_8: ArrayOfSpell,
	#[serde(default, rename = "9")]
	level_9: ArrayOfSpell,
	#[serde(default, rename = "1e")]
	level_1e: ArrayOfSpell,
	#[serde(default, rename = "2e")]
	level_2e: ArrayOfSpell,
	#[serde(default, rename = "3e")]
	level_3e: ArrayOfSpell,
	#[serde(default, rename = "4e")]
	level_4e: ArrayOfSpell,
	#[serde(default, rename = "5e")]
	level_5e: ArrayOfSpell,
	#[serde(default, rename = "6e")]
	level_6e: ArrayOfSpell,
	#[serde(default, rename = "7e")]
	level_7e: ArrayOfSpell,
	#[serde(default, rename = "8e")]
	level_8e: ArrayOfSpell,
	#[serde(default, rename = "9e")]
	level_9e: ArrayOfSpell,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntrySpellcastingRecharge {
	#[serde(default, rename = "1")]
	level_1: ArrayOfSpell,
	#[serde(default, rename = "2")]
	level_2: ArrayOfSpell,
	#[serde(default, rename = "3")]
	level_3: ArrayOfSpell,
	#[serde(default, rename = "4")]
	level_4: ArrayOfSpell,
	#[serde(default, rename = "5")]
	level_5: ArrayOfSpell,
	#[serde(default, rename = "6")]
	level_6: ArrayOfSpell,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntrySpellcastingSpellsLevel0 {
	#[serde(default)]
	spells: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntrySpellcastingSpells {
	#[serde(default, rename = "0")]
	level_0: EntrySpellcastingSpellsLevel0,
	#[serde(default, rename = "1")]
	level_1: EntrySpellcastingLevel1To9,
	#[serde(default, rename = "2")]
	level_2: EntrySpellcastingLevel1To9,
	#[serde(default, rename = "3")]
	level_3: EntrySpellcastingLevel1To9,
	#[serde(default, rename = "4")]
	level_4: EntrySpellcastingLevel1To9,
	#[serde(default, rename = "5")]
	level_5: EntrySpellcastingLevel1To9,
	#[serde(default, rename = "6")]
	level_6: EntrySpellcastingLevel1To9,
	#[serde(default, rename = "7")]
	level_7: EntrySpellcastingLevel1To9,
	#[serde(default, rename = "8")]
	level_8: EntrySpellcastingLevel1To9,
	#[serde(default, rename = "9")]
	level_9: EntrySpellcastingLevel1To9,
}

#[derive(Debug, Deserialize)]
pub enum EntrySpellcastingHidden {
	#[serde(rename = "constant")]
	Constant,
	#[serde(rename = "will")]
	Will,
	#[serde(rename = "rest")]
	Rest,
	#[serde(rename = "restLong")]
	RestLong,
	#[serde(rename = "daily")]
	Daily,
	#[serde(rename = "weekly")]
	Weekly,
	#[serde(rename = "monthly")]
	Monthly,
	#[serde(rename = "yearly")]
	Yearly,
	#[serde(rename = "ritual")]
	Ritual,
	#[serde(rename = "spells")]
	Spells,
	#[serde(rename = "charges")]
	Charges,
	#[serde(rename = "recharge")]
	Recharge,
	#[serde(rename = "legendary")]
	Legendary,
}

/**
 * Shouldn't have default
 */
#[derive(Debug, Default, Deserialize)]
pub enum EntrySpellcastingDisplayAs {
	#[serde(rename = "trait")]
	Trait,
	#[serde(rename = "action")]
	Action,
	#[serde(rename = "bonus")]
	Bonus,
	#[serde(rename = "reaction")]
	Reaction,
	#[serde(rename = "legendary")]
	Legendary,
	#[serde(rename = "mythic")]
	Mythic,
	#[default]
	None,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntrySpellcasting {
	#[serde(default)]
	name: String,
	// Always "spellcasting"
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
	#[serde(default, rename = "headerEntries")]
	header_entries: Vec<Entry>,
	#[serde(default)]
	constant: ArrayOfSpell,
	#[serde(default)]
	will: ArrayOfSpell,
	#[serde(default)]
	ritual: ArrayOfSpell,
	#[serde(default)]
	rest: EntrySpellcastingFrequency,
	#[serde(default, rename = "restLong")]
	rest_long: EntrySpellcastingFrequency,
	#[serde(default)]
	daily: EntrySpellcastingFrequency,
	#[serde(default)]
	weekly: EntrySpellcastingFrequency,
	#[serde(default)]
	monthly: EntrySpellcastingFrequency,
	#[serde(default)]
	yearly: EntrySpellcastingFrequency,
	#[serde(default)]
	charges: EntrySpellcastingFrequency,
	#[serde(default)]
	recharge: EntrySpellcastingRecharge,
	#[serde(default)]
	legendary: EntrySpellcastingFrequency,
	#[serde(default)]
	spells: EntrySpellcastingSpells,
	// Allows the above properties to be specified, but not rendered. Useful if e.g. a creature can only cast one spell, and this is rendered in the header line.
	#[serde(default)]
	hidden: Vec<EntrySpellcastingHidden>,
	#[serde(default, rename = "footerEntries")]
	footer_entries: Vec<Entry>,
	#[serde(default)]
	ability: AbilityScoreAbbreviation,
	// Implicitly "trait"
	#[serde(default, rename = "displayAs")]
	display_as: EntrySpellcastingDisplayAs,
	// The UID of an item which provides the charges required to cast any "charges" spells.
	#[serde(default, rename = "chargesItem")]
	charges_item: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntrySpellcastingLevel1To9 {
	#[serde(default)]
	lower: f64,
	#[serde(default)]
	slots: f64,
	#[serde(default)]
	spells: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryFlowchart {
	#[serde(default)]
	name: String,
	// Always "attack"
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
	#[serde(default)]
	blocks: Vec<EntryEntry>,
}

#[derive(Debug, Default, Deserialize)]
pub struct EntryFlowBlock {
	#[serde(default)]
	name: String,
	// Always "flowBlock"
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
	#[serde(default)]
	entries: Vec<EntryEntry>,
}

/**
 * "patternProperties": {
 *		"^amount\\d+$": {
 *			"type": "number"
 *		}
 *	},
 */
#[derive(Debug, Default, Deserialize)]
pub struct EntryIngredient {
	#[serde(default)]
	name: String,
	// Always "ingrediant"
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
	#[serde(default)]
	entry: Entry,
}

/**
 * has field: "wrapped": {}
 */
#[derive(Debug, Default, Deserialize)]
pub struct EntryWrapped {
	#[serde(default)]
	name: String,
	// Always "wrapper"
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
}

/**
 * An entry can be any one of the above types including either a string or an integer.
 */
#[derive(Debug, Default, Deserialize)]
pub struct Entry {}
