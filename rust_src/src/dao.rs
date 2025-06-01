use std::collections::HashMap;

pub mod actions_dao;
pub mod backgrounds_dao;
pub mod feats_dao;
pub mod items_dao;
pub mod languages_dao;
pub mod common;
pub mod races_dao;
pub mod skills_dao;
pub mod spells_dao;

use actions_dao::Action;
use backgrounds_dao::Background;
use feats_dao::Feat;
use items_dao::Item;
use languages_dao::Language;
use races_dao::Race;
use skills_dao::Skill;
use spells_dao::Spell;

pub struct DAO {
	actions: HashMap<String, Action>,
	backgrounds: HashMap<String, Background>,
	feats: HashMap<String, Feat>,
	items: HashMap<String, Item>,
	languages: HashMap<String, Language>,
	races: HashMap<String, Race>,
	skills: HashMap<String, Skill>,
	spells: HashMap<String, Spell>,
}
