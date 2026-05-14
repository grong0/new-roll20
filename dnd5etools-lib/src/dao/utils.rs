use serde::{
	Deserialize, Deserializer,
	de::{self, Visitor},
};

use crate::dao::utils_edition::Edition;

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

	return deserializer.deserialize_string(ReprintedAsVisitor);
}

#[derive(Debug, Default, Deserialize)]
struct PrerequisiteLevel {}

#[derive(Debug, Default, Deserialize)]
struct PrerequisitePact {}

#[derive(Debug, Default, Deserialize)]
struct PrerequisitePatron {}

pub type Prerequisites = Vec<Prerequisite>;

#[derive(Debug, Default, Deserialize)]
pub struct Prerequisite {
	#[serde(default)]
	level: PrerequisiteLevel,
	pact: PrerequisitePact,
	patron: PrerequisitePatron,
	spell: PrerequisiteSpell,
	feat: Vec<String>,
	feat_category, Vec<DataFeatCategory>,
	exclusive_feat_category: Vec<DataFeatCategory>,
	#[serde(rename = "optionalfeature")]
	optional_feature: Vec<String>,
	feature: Vec<String>,
	item: Vec<String>,
	item_type: ItemType
}

#[derive(Debug, Default, Deserialize)]
pub struct DataFeatCategory {}
