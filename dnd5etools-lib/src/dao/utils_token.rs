use serde::Deserialize;

use crate::dao::utils::{Page, Source};

/**
 * A name/source pair used to construct a token URL for the entity.
 * For example `"name": "Goblin", "source": "MM"` for a creature token would refer to the MM Goblin's token.
 */
#[derive(Debug, Default, Deserialize)]
pub struct Token {
	name: String,
	source: Source
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AltArtTokenTag {
	TopDown,
}

pub type AltArtTokenTags = Vec<AltArtTokenTag>;

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AltArtItemSite {
	name: String,
	source: Source,
	page: Page,
	token_credit: String,
	token_custom: bool,
	token_tags: AltArtTokenTags,
}

pub type AltArtItem = AltArtItemSite;

pub type AltArt = Vec<AltArtItem>;
