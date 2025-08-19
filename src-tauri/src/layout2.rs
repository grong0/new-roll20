use std::collections::HashMap;

use crate::{
	components::{
		class_badge, proficiency_expertise, proficiency_halfproficient, proficiency_proficient, proficiency_untrained, workspace_actions,
		workspace_actions_action, workspace_actions_bonusaction, workspace_actions_item, workspace_actions_limiteduse,
		workspace_actions_other, workspace_actions_reaction, workspace_feats, workspace_feats_class, workspace_feats_classheader,
		workspace_feats_general, workspace_feats_item, workspace_feats_race, workspace_inventory, workspace_inventory_attunement,
		workspace_inventory_attunmentitem, workspace_inventory_container, workspace_inventory_containeritem, workspace_inventory_equipment,
		workspace_inventory_extratabbutton, workspace_inventory_otherpossessions, workspace_spells_table,
	},
	frontend_functions::ability_score_to_modifier,
};

#[tauri::command]
pub fn player_name() -> String {
	return "Test Character".to_string();
}

#[tauri::command]
pub fn player_race() -> String {
	return "Human".to_string();
}

#[tauri::command]
pub fn player_level() -> String {
	return "13".to_string();
}

#[tauri::command]
pub fn player_hitpoints_max() -> String {
	return "38".to_string();
}

#[tauri::command]
pub fn player_hitpoints_current() -> String {
	return "27".to_string();
}

#[tauri::command]
pub fn player_hitpoints_percent() -> String {
	return format!("{}%", (250.0 as f32 / 304.0 as f32).round() * 100 as f32);
}

#[tauri::command]
pub fn player_hitpoints_temp() -> String {
	return "12".to_string();
}

#[tauri::command]
pub fn player_armorclass() -> String {
	return "15".to_string();
}

#[tauri::command]
pub fn player_initiative() -> String {
	return "5".to_string();
}

#[tauri::command]
pub fn player_speed_walking() -> String {
	return "30".to_string();
}

#[tauri::command]
pub fn player_class_level() -> String {
	return "13".to_string();
}

#[tauri::command]
pub fn player_classes() -> String {
	let classes = vec![
		// String::from("artificer"),
		// String::from("barbarian"),
		// String::from("bard"),
		// String::from("cleric"),
		// String::from("druid"),
		String::from("fighter"),
		// String::from("monk"),
		// String::from("paladin"),
		// String::from("ranger"),
		// String::from("rogue"),
		// String::from("sorcerer"),
		// String::from("warlock"),
		String::from("wizard"),
	];

	let mut content = String::new();
	for player_class in classes {
		content += class_badge(&player_class.to_ascii_uppercase().to_string(), &player_class.to_string()).as_str();
	}
	return content;
}

#[tauri::command]
pub fn player_abilities_str_score() -> String {
	return 11.to_string();
}
#[tauri::command]
pub fn player_abilities_dex_score() -> String {
	return 20.to_string();
}
#[tauri::command]
pub fn player_abilities_con_score() -> String {
	return 14.to_string();
}
#[tauri::command]
pub fn player_abilities_int_score() -> String {
	return 20.to_string();
}
#[tauri::command]
pub fn player_abilities_wis_score() -> String {
	return 11.to_string();
}
#[tauri::command]
pub fn player_abilities_cha_score() -> String {
	return 11.to_string();
}

#[tauri::command]
pub fn player_abilities_str_modifier() -> String {
	return ability_score_to_modifier(11).to_string();
}
#[tauri::command]
pub fn player_abilities_dex_modifier() -> String {
	return ability_score_to_modifier(20).to_string();
}
#[tauri::command]
pub fn player_abilities_con_modifier() -> String {
	return ability_score_to_modifier(14).to_string();
}
#[tauri::command]
pub fn player_abilities_int_modifier() -> String {
	return ability_score_to_modifier(20).to_string();
}
#[tauri::command]
pub fn player_abilities_wis_modifier() -> String {
	return ability_score_to_modifier(11).to_string();
}
#[tauri::command]
pub fn player_abilities_cha_modifier() -> String {
	return ability_score_to_modifier(11).to_string();
}

#[tauri::command]
pub fn player_skills_acrobatics_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_animalhandling_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_arcana_status() -> String {
	return proficiency_expertise(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_athletics_status() -> String {
	return proficiency_halfproficient(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_deception_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_history_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_insight_status() -> String {
	return proficiency_proficient(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_intimidation_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_investigation_status() -> String {
	return proficiency_proficient(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_medicine_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_nature_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_perception_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_performance_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_persuasion_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_religion_status() -> String {
	return proficiency_proficient(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_slightofhand_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_stealth_status() -> String {
	return proficiency_untrained(&String::from("right"));
}
#[tauri::command]
pub fn player_skills_survival_status() -> String {
	return proficiency_untrained(&String::from("right"));
}

#[tauri::command]
pub fn player_skills_acrobatics_bonus() -> String {
	return 5.to_string();
}
#[tauri::command]
pub fn player_skills_animalhandling_bonus() -> String {
	return 0.to_string();
}
#[tauri::command]
pub fn player_skills_arcana_bonus() -> String {
	return 15.to_string();
}
#[tauri::command]
pub fn player_skills_athletics_bonus() -> String {
	return 2.to_string();
}
#[tauri::command]
pub fn player_skills_deception_bonus() -> String {
	return 0.to_string();
}
#[tauri::command]
pub fn player_skills_history_bonus() -> String {
	return 5.to_string();
}
#[tauri::command]
pub fn player_skills_insight_bonus() -> String {
	return 5.to_string();
}
#[tauri::command]
pub fn player_skills_intimidation_bonus() -> String {
	return 0.to_string();
}
#[tauri::command]
pub fn player_skills_investigation_bonus() -> String {
	return 10.to_string();
}
#[tauri::command]
pub fn player_skills_medicine_bonus() -> String {
	return 0.to_string();
}
#[tauri::command]
pub fn player_skills_nature_bonus() -> String {
	return 5.to_string();
}
#[tauri::command]
pub fn player_skills_perception_bonus() -> String {
	return 0.to_string();
}
#[tauri::command]
pub fn player_skills_performance_bonus() -> String {
	return 0.to_string();
}
#[tauri::command]
pub fn player_skills_persuasion_bonus() -> String {
	return 0.to_string();
}
#[tauri::command]
pub fn player_skills_religion_bonus() -> String {
	return 10.to_string();
}
#[tauri::command]
pub fn player_skills_slightofhand_bonus() -> String {
	return 5.to_string();
}
#[tauri::command]
pub fn player_skills_stealth_bonus() -> String {
	return 5.to_string();
}
#[tauri::command]
pub fn player_skills_survival_bonus() -> String {
	return 0.to_string();
}

#[tauri::command]
pub fn player_senses_perception() -> String {
	return "10".to_string();
}

#[tauri::command]
pub fn player_senses_investigation() -> String {
	return "20".to_string();
}

#[tauri::command]
pub fn player_senses_insight() -> String {
	return "15".to_string();
}

#[tauri::command]
pub fn player_proficiencies_armor() -> String {
	return "Light Armor, Medium Armor, Shields".to_string();
}

#[tauri::command]
pub fn player_proficiencies_weapons() -> String {
	return "Crossbow, Light, Dagger, Dart, Martial Weapons, Quarterstaff, Sling".to_string();
}

#[tauri::command]
pub fn player_proficiencies_tools() -> String {
	return "None".to_string();
}

#[tauri::command]
pub fn player_proficiencies_languages() -> String {
	return "Common, Deep Speech, Draconic, Infernal".to_string();
}

struct Damage {
	dice: String,
	damage_type: String,
}

struct Action {
	name: String,
	action_type: String,
	range: String,
	hitdc: String,
	damage: Damage,
	notes: String,
}

#[tauri::command]
pub fn player_actions() -> String {
	let actions = vec![
		Action {
			name: "Ray of Frost".to_string(),
			action_type: "Cantrip - Wizard".to_string(),
			range: 60.to_string(),
			hitdc: 11.to_string(),
			damage: Damage { dice: "4d8".to_string(), damage_type: "cold".to_string() },
			notes: "V/S".to_string(),
		},
		Action {
			name: "Unarmed Strike".to_string(),
			action_type: "Melee Attack".to_string(),
			range: 5.to_string(),
			hitdc: 5.to_string(),
			damage: Damage { dice: "0".to_string(), damage_type: "bludgeoning".to_string() },
			notes: "".to_string(),
		},
		Action {
			name: "Melf's Minute Meteors".to_string(),
			action_type: "3rd Level - Wizard".to_string(),
			range: "Self".to_string(),
			hitdc: "DEX 14".to_string(),
			damage: Damage { dice: "2d6".to_string(), damage_type: "fire".to_string() },
			notes: "Concentration, V/S/M".to_string(),
		},
		Action {
			name: "Fireball".to_string(),
			action_type: "3rd Level - Wizard".to_string(),
			range: "150".to_string(),
			hitdc: "DEX 14".to_string(),
			damage: Damage { dice: "8d6".to_string(), damage_type: "fire".to_string() },
			notes: "V/S/M".to_string(),
		},
	]
	.iter()
	.map(|i| {
		workspace_actions_item(&i.name, &i.action_type, &i.range, &i.hitdc, &i.damage.dice, &i.damage.damage_type, &i.notes)
			.chars()
			.collect::<Vec<_>>()
	})
	.flatten()
	.collect();
	let bonus_actions = vec![Action {
		name: "Ray of Frost".to_string(),
		action_type: "Cantrip - Wizard".to_string(),
		range: 60.to_string(),
		hitdc: 11.to_string(),
		damage: Damage { dice: "4d8".to_string(), damage_type: "cold".to_string() },
		notes: "V/S".to_string(),
	}]
	.iter()
	.map(|i| {
		workspace_actions_item(&i.name, &i.action_type, &i.range, &i.hitdc, &i.damage.dice, &i.damage.damage_type, &i.notes)
			.chars()
			.collect::<Vec<_>>()
	})
	.flatten()
	.collect();
	let reactions = vec![Action {
		name: "Unarmed Strike".to_string(),
		action_type: "Melee Attack".to_string(),
		range: 5.to_string(),
		hitdc: 5.to_string(),
		damage: Damage { dice: "0".to_string(), damage_type: "bludgeoning".to_string() },
		notes: "".to_string(),
	}]
	.iter()
	.map(|i| {
		workspace_actions_item(&i.name, &i.action_type, &i.range, &i.hitdc, &i.damage.dice, &i.damage.damage_type, &i.notes)
			.chars()
			.collect::<Vec<_>>()
	})
	.flatten()
	.collect();
	let other_actions = vec![Action {
		name: "Melf's Minute Meteors".to_string(),
		action_type: "3rd Level - Wizard".to_string(),
		range: "Self".to_string(),
		hitdc: "DEX 14".to_string(),
		damage: Damage { dice: "2d6".to_string(), damage_type: "fire".to_string() },
		notes: "Concentration, V/S/M".to_string(),
	}]
	.iter()
	.map(|i| {
		workspace_actions_item(&i.name, &i.action_type, &i.range, &i.hitdc, &i.damage.dice, &i.damage.damage_type, &i.notes)
			.chars()
			.collect::<Vec<_>>()
	})
	.flatten()
	.collect();
	let limited_use_actions = vec![Action {
		name: "Fireball".to_string(),
		action_type: "3rd Level - Wizard".to_string(),
		range: "150".to_string(),
		hitdc: "DEX 14".to_string(),
		damage: Damage { dice: "8d6".to_string(), damage_type: "fire".to_string() },
		notes: "V/S/M".to_string(),
	}]
	.iter()
	.map(|i| {
		workspace_actions_item(&i.name, &i.action_type, &i.range, &i.hitdc, &i.damage.dice, &i.damage.damage_type, &i.notes)
			.chars()
			.collect::<Vec<_>>()
	})
	.flatten()
	.collect();

	let actions_in_combat =
		String::from("Attack, Cast a Spell, Dash, Disengage, Dodge, Grapple, Help, Hide, Improvise, Ready, Search, Shove, Use an Object");
	let bonus_actions_in_combat = String::from("Nothing");
	let reactions_in_combat = String::from("Nothing");
	let other_actions_in_combat = String::from("Nothing");
	let limited_use_actions_in_combat = String::from("Nothing");

	return workspace_actions(
		&actions,
		&bonus_actions,
		&reactions,
		&other_actions,
		&limited_use_actions,
		&actions_in_combat,
		&bonus_actions_in_combat,
		&reactions_in_combat,
		&other_actions_in_combat,
		&limited_use_actions_in_combat,
	);
}

#[tauri::command]
pub fn player_actions_action() -> String {
	let actions = vec![
		Action {
			name: "Ray of Frost".to_string(),
			action_type: "Cantrip - Wizard".to_string(),
			range: 60.to_string(),
			hitdc: 11.to_string(),
			damage: Damage { dice: "4d8".to_string(), damage_type: "cold".to_string() },
			notes: "V/S".to_string(),
		},
		Action {
			name: "Unarmed Strike".to_string(),
			action_type: "Melee Attack".to_string(),
			range: 5.to_string(),
			hitdc: 5.to_string(),
			damage: Damage { dice: "0".to_string(), damage_type: "bludgeoning".to_string() },
			notes: "".to_string(),
		},
		Action {
			name: "Melf's Minute Meteors".to_string(),
			action_type: "3rd Level - Wizard".to_string(),
			range: "Self".to_string(),
			hitdc: "DEX 14".to_string(),
			damage: Damage { dice: "2d6".to_string(), damage_type: "fire".to_string() },
			notes: "Concentration, V/S/M".to_string(),
		},
		Action {
			name: "Fireball".to_string(),
			action_type: "3rd Level - Wizard".to_string(),
			range: "150".to_string(),
			hitdc: "DEX 14".to_string(),
			damage: Damage { dice: "8d6".to_string(), damage_type: "fire".to_string() },
			notes: "V/S/M".to_string(),
		},
	];
	let mut content = String::new();
	for action in actions {
		content += workspace_actions_item(
			&action.name,
			&action.action_type,
			&action.range,
			&action.hitdc,
			&action.damage.dice,
			&action.damage.damage_type,
			&action.notes,
		)
		.as_str();
	}

	let actions_in_combat =
		String::from("Attack, Cast a Spell, Dash, Disengage, Dodge, Grapple, Help, Hide, Improvise, Ready, Search, Shove, Use an Object");
	return workspace_actions_action(&content, &actions_in_combat);
}

#[tauri::command]
pub fn player_actions_bonusaction() -> String {
	let actions = vec![Action {
		name: "Melf's Minute Meteors".to_string(),
		action_type: "3rd Level - Wizard".to_string(),
		range: "Self".to_string(),
		hitdc: "DEX 14".to_string(),
		damage: Damage { dice: "2d6".to_string(), damage_type: "fire".to_string() },
		notes: "Concentration, V/S/M".to_string(),
	}];
	let mut content = String::new();
	for action in actions {
		content += workspace_actions_item(
			&action.name,
			&action.action_type,
			&action.range,
			&action.hitdc,
			&action.damage.dice,
			&action.damage.damage_type,
			&action.notes,
		)
		.as_str();
	}
	let actions_in_combat = String::from("Nothing");
	return workspace_actions_bonusaction(&content, &actions_in_combat);
}

#[tauri::command]
pub fn player_actions_reaction() -> String {
	let actions = vec![Action {
		name: "Unarmed Strike".to_string(),
		action_type: "Melee Attack".to_string(),
		range: 5.to_string(),
		hitdc: 5.to_string(),
		damage: Damage { dice: "0".to_string(), damage_type: "bludgeoning".to_string() },
		notes: "".to_string(),
	}];
	let mut content = String::new();
	for action in actions {
		content += workspace_actions_item(
			&action.name,
			&action.action_type,
			&action.range,
			&action.hitdc,
			&action.damage.dice,
			&action.damage.damage_type,
			&action.notes,
		)
		.as_str();
	}
	let actions_in_combat = String::from("Nothing");
	return workspace_actions_reaction(&content, &actions_in_combat);
}

#[tauri::command]
pub fn player_actions_other() -> String {
	let actions = vec![Action {
		name: "Ray of Frost".to_string(),
		action_type: "Cantrip - Wizard".to_string(),
		range: 60.to_string(),
		hitdc: 11.to_string(),
		damage: Damage { dice: "4d8".to_string(), damage_type: "cold".to_string() },
		notes: "V/S".to_string(),
	}];
	let mut content = String::new();
	for action in actions {
		content += workspace_actions_item(
			&action.name,
			&action.action_type,
			&action.range,
			&action.hitdc,
			&action.damage.dice,
			&action.damage.damage_type,
			&action.notes,
		)
		.as_str();
	}
	let actions_in_combat = String::from("Nothing");
	return workspace_actions_other(&content, &actions_in_combat);
}

#[tauri::command]
pub fn player_actions_limiteduse() -> String {
	let actions = vec![Action {
		name: "Fireball".to_string(),
		action_type: "3rd Level - Wizard".to_string(),
		range: "150".to_string(),
		hitdc: "DEX 14".to_string(),
		damage: Damage { dice: "8d6".to_string(), damage_type: "fire".to_string() },
		notes: "V/S/M".to_string(),
	}];
	let mut content = String::new();
	for action in actions {
		content += workspace_actions_item(
			&action.name,
			&action.action_type,
			&action.range,
			&action.hitdc,
			&action.damage.dice,
			&action.damage.damage_type,
			&action.notes,
		)
		.as_str();
	}
	let actions_in_combat = String::from("Nothing");
	return workspace_actions_limiteduse(&content, &actions_in_combat);
}

#[tauri::command]
pub fn player_actionsincombat_action() -> String {
	return "Attack, Cast a Spell, Dash, Disengage, Dodge, Grapple, Help, Hide, Improvise, Ready, Search, Shove, Use an Object".to_string();
}

#[tauri::command]
pub fn player_actionsincombat_bonusaction() -> String {
	return "Nothing".to_string();
}

#[tauri::command]
pub fn player_actionsincombat_reaction() -> String {
	return "Nothing".to_string();
}

#[tauri::command]
pub fn player_actionsincombat_other() -> String {
	return "Nothing".to_string();
}

#[tauri::command]
pub fn player_actionsincombat_limiteduse() -> String {
	return "Nothing".to_string();
}

struct MockEntry {
	title: String,
	content: Vec<String>,
}

struct MockEntryParent {
	title: String,
	source: String,
	page: u64,
	content: Vec<MockEntry>,
}

#[tauri::command]
pub fn player_feats() -> String {
	let fighter_entries: Vec<MockEntryParent> = vec![
		MockEntryParent {
			title: String::from("Fighting Style"),
			source: String::from("PHB"),
			page: 91,
			content: vec![MockEntry {
				title: String::new(),
				content: vec![String::from("You gain a Fighting Style feat of your choice, and whenever you gain a Fighter level, you can replace the feat you chose with a different Fighting Style feat.")],
			}],
		},
		MockEntryParent {
			title: String::from("Second Wind"),
			source: String::from("PHB"),
			page: 91,
			content: vec![MockEntry {
				title: String::new(),
				content: vec![
					String::from(
						"As a Bonus Action, you can draw upon a limited well of physical and mental stamina and regain 1d10+5 HP.",
					),
					String::from("You can use this 3 times per Long Rest, and can regain one expended use when you finish a Short Rest."),
				],
			}],
		},
	];
	let wizard_entries: Vec<MockEntryParent> = vec![
		MockEntryParent {
			title: String::from("Spellcasting"),
			source: String::from("PHB"),
			page: 114,
			content: vec![MockEntry {title: String::new(), content: vec![String::from("You can cast prepared wizard spells using INT as your spellcasting modifier (Spell DC 15, Spell Attack +7) and wizard spells in your spellbook as rituals if they have the ritual tag. You can use an arcane focus as a spellcasting focus.")]}]
		},
		MockEntryParent {
			title: String::from("Arcane Recovery"),
			source: String::from("PHB"),
			page: 115,
			content: vec![
				MockEntry {
					title: String::new(),
					content: vec![String::from("You have learned to regain some of your magical energy by studying your spellbook. Once per day when you finish a short rest, you can choose expended spell slots to recover. The spell slots can have a combined level that is equal to or less than half your wizard level (rounded up), and none of the slots can be 6th level or higher.")]
				},
				MockEntry {
					title: String::new(),
					content: vec![String::from("For example, if you’re a 4th-level wizard, you can recover up to two levels worth of spell slots. You can recover either a 2nd-level spell slot or two 1st-level spell slots.")]
				},
			]
		},
		MockEntryParent {
			title: String::from("Evocation Savent"),
			source: String::from("PHB"),
			page: 117,
			content: vec![MockEntry {title: String::new(), content: vec![String::from("Beginning when you select this school at 2nd level, the gold and time you must spend to copy an evocation spell into your spellbook is halved.")]}]
		},
	];

	let human_entries: Vec<MockEntryParent> = vec![
		MockEntryParent {
			title: String::from("Ability Score Increase"),
			source: String::from("BR"),
			page: 31,
			content: vec![MockEntry { title: String::new(), content: vec![String::from("Your ability scores each increase by 1.")] }],
		},
		MockEntryParent {
			title: String::from("Languages"),
			source: String::from("BR"),
			page: 31,
			content: vec![MockEntry {
				title: String::new(),
				content: vec![String::from("You can speak, read, and write Common and one extra language.")],
			}],
		},
	];

	let feat_entries: Vec<MockEntryParent> = vec![
		MockEntryParent { title: String::from("Elemental Adept"), source: String::from("PHB"), page: 203, content: vec![
			MockEntry {
				title: String::from("Ability Score Increase."), content: vec![String::from("Int., Wis., or Cha. increased by 1.")]
			},
			MockEntry {
				title: String::from("Energy Mastery."), content: vec![String::from("Choose one of the following damage types: Acid, Cold, Fire, Lightning, or Thunder. Spells you cast ignore Resistance to damage of the chosen type. In addition, when you roll damage for a spell you cast that deals damage of that type, you can treat any 1 on a damage die as a 2.")]
			},
			MockEntry {
				title: String::from("Repeatable."), content: vec![String::from("You can take this feat more than once, but you must choose a different damage type each time for Energy Mastery.")]
			}
		] },
		MockEntryParent { title: String::from("Grappler"), source: String::from("PHB"), page: 204, content: vec![
			MockEntry {
				title: String::from("Ability Score Increase."), content: vec![String::from("Increase your Str. or Dex. by 1.")]
			},
			MockEntry {
				title: String::from("Punch and Grab."), content: vec![String::from("On your turn, when you hit a creature with an Unarmed Strike you can use both the Damage and the Grapple option. You can use this benefit only once per turn.")]
			},
			MockEntry {
				title: String::from("Attack Advantage."), content: vec![String::from("You have Advantage on attack rolls against a creature Grappled by you.")]
			},
			MockEntry {
				title: String::from("Fast Wrestler."), content: vec![String::from("You don't have to spend extra movement to move a creature Grappled by you if the creature is your size or smaller.")]
			},
		] },
	];

	let fighter_name = String::from("Fighter");
	let wizard_name = String::from("Wizard");

	let mut class_content = String::new();
	class_content += workspace_feats_classheader(&fighter_name).as_str();
	for entry in fighter_entries {
		let content = entry.content.iter().map(|i| format!("<h4>{}</h4>{}", i.title, i.content.join(""))).collect();
		class_content += workspace_feats_item(&entry.title, &entry.source, &entry.page, &content).as_str();
	}
	class_content += workspace_feats_classheader(&wizard_name).as_str();
	for entry in wizard_entries {
		let content = entry.content.iter().map(|i| format!("<h4>{}</h4>{}", i.title, i.content.join(""))).collect();
		class_content += workspace_feats_item(&entry.title, &entry.source, &entry.page, &content).as_str();
	}

	let mut racial_content = String::new();
	for entry in human_entries {
		let content = entry.content.iter().map(|i| format!("<h4>{}</h4>{}", i.title, i.content.join(""))).collect();
		racial_content += workspace_feats_item(&entry.title, &entry.source, &entry.page, &content).as_str();
	}

	let mut general_content = String::new();
	for entry in feat_entries {
		let content = entry.content.iter().map(|i| format!("<h4>{}</h4>{}", i.title, i.content.join(""))).collect();
		general_content += workspace_feats_item(&entry.title, &entry.source, &entry.page, &content).as_str();
	}

	return workspace_feats(&class_content, &racial_content, &general_content);
}

#[tauri::command]
pub fn player_feats_class() -> String {
	let fighter_entries: Vec<MockEntryParent> = vec![
		MockEntryParent {
			title: String::from("Fighting Style"),
			source: String::from("PHB"),
			page: 91,
			content: vec![MockEntry {
				title: String::new(),
				content: vec![String::from("You gain a Fighting Style feat of your choice, and whenever you gain a Fighter level, you can replace the feat you chose with a different Fighting Style feat.")],
			}],
		},
		MockEntryParent {
			title: String::from("Second Wind"),
			source: String::from("PHB"),
			page: 91,
			content: vec![MockEntry {
				title: String::new(),
				content: vec![
					String::from(
						"As a Bonus Action, you can draw upon a limited well of physical and mental stamina and regain 1d10+5 HP.",
					),
					String::from("You can use this 3 times per Long Rest, and can regain one expended use when you finish a Short Rest."),
				],
			}],
		},
	];
	let wizard_entries: Vec<MockEntryParent> = vec![
		MockEntryParent {
			title: String::from("Spellcasting"),
			source: String::from("PHB"),
			page: 114,
			content: vec![MockEntry {title: String::new(), content: vec![String::from("You can cast prepared wizard spells using INT as your spellcasting modifier (Spell DC 15, Spell Attack +7) and wizard spells in your spellbook as rituals if they have the ritual tag. You can use an arcane focus as a spellcasting focus.")]}]
		},
		MockEntryParent {
			title: String::from("Arcane Recovery"),
			source: String::from("PHB"),
			page: 115,
			content: vec![
				MockEntry {
					title: String::new(),
					content: vec![String::from("You have learned to regain some of your magical energy by studying your spellbook. Once per day when you finish a short rest, you can choose expended spell slots to recover. The spell slots can have a combined level that is equal to or less than half your wizard level (rounded up), and none of the slots can be 6th level or higher.")]
				},
				MockEntry {
					title: String::new(),
					content: vec![String::from("For example, if you’re a 4th-level wizard, you can recover up to two levels worth of spell slots. You can recover either a 2nd-level spell slot or two 1st-level spell slots.")]
				},
			]
		},
		MockEntryParent {
			title: String::from("Evocation Savent"),
			source: String::from("PHB"),
			page: 117,
			content: vec![MockEntry {title: String::new(), content: vec![String::from("Beginning when you select this school at 2nd level, the gold and time you must spend to copy an evocation spell into your spellbook is halved.")]}]
		},
	];

	let fighter_name = String::from("Fighter");
	let wizard_name = String::from("Wizard");

	let mut content = String::new();
	content += workspace_feats_classheader(&fighter_name).as_str();
	for entry in fighter_entries {
		let entry_content = entry.content.iter().map(|i| format!("<h4>{}</h4>{}", i.title, i.content.join(""))).collect();
		content += workspace_feats_item(&entry.title, &entry.source, &entry.page, &entry_content).as_str();
	}
	content += workspace_feats_classheader(&wizard_name).as_str();
	for entry in wizard_entries {
		let entry_content = entry.content.iter().map(|i| format!("<h4>{}</h4>{}", i.title, i.content.join(""))).collect();
		content += workspace_feats_item(&entry.title, &entry.source, &entry.page, &entry_content).as_str();
	}

	return workspace_feats_class(&content);
}

#[tauri::command]
pub fn player_feats_race() -> String {
	let entries: Vec<MockEntryParent> = vec![
		MockEntryParent {
			title: String::from("Ability Score Increase"),
			source: String::from("BR"),
			page: 31,
			content: vec![MockEntry { title: String::new(), content: vec![String::from("Your ability scores each increase by 1.")] }],
		},
		MockEntryParent {
			title: String::from("Languages"),
			source: String::from("BR"),
			page: 31,
			content: vec![MockEntry {
				title: String::new(),
				content: vec![String::from("You can speak, read, and write Common and one extra language.")],
			}],
		},
	];

	let mut content = String::new();
	for entry in entries {
		let entry_content = entry.content.iter().map(|i| format!("<h4>{}</h4>{}", i.title, i.content.join(""))).collect();
		content += workspace_feats_item(&entry.title, &entry.source, &entry.page, &entry_content).as_str();
	}

	return workspace_feats_race(&content);
}

#[tauri::command]
pub fn player_feats_general() -> String {
	let entries: Vec<MockEntryParent> = vec![
		MockEntryParent { title: String::from("Elemental Adept"), source: String::from("PHB"), page: 203, content: vec![
			MockEntry {
				title: String::from("Ability Score Increase."), content: vec![String::from("Int., Wis., or Cha. increased by 1.")]
			},
			MockEntry {
				title: String::from("Energy Mastery."), content: vec![String::from("Choose one of the following damage types: Acid, Cold, Fire, Lightning, or Thunder. Spells you cast ignore Resistance to damage of the chosen type. In addition, when you roll damage for a spell you cast that deals damage of that type, you can treat any 1 on a damage die as a 2.")]
			},
			MockEntry {
				title: String::from("Repeatable."), content: vec![String::from("You can take this feat more than once, but you must choose a different damage type each time for Energy Mastery.")]
			}
		] },
		MockEntryParent { title: String::from("Grappler"), source: String::from("PHB"), page: 204, content: vec![
			MockEntry {
				title: String::from("Ability Score Increase."), content: vec![String::from("Increase your Str. or Dex. by 1.")]
			},
			MockEntry {
				title: String::from("Punch and Grab."), content: vec![String::from("On your turn, when you hit a creature with an Unarmed Strike you can use both the Damage and the Grapple option. You can use this benefit only once per turn.")]
			},
			MockEntry {
				title: String::from("Attack Advantage."), content: vec![String::from("You have Advantage on attack rolls against a creature Grappled by you.")]
			},
			MockEntry {
				title: String::from("Fast Wrestler."), content: vec![String::from("You don't have to spend extra movement to move a creature Grappled by you if the creature is your size or smaller.")]
			},
		] },
	];

	let mut content = String::new();
	for entry in entries {
		let entry_content = entry.content.iter().map(|i| format!("<h4>{}</h4>{}", i.title, i.content.join(""))).collect();
		content += workspace_feats_item(&entry.title, &entry.source, &entry.page, &entry_content).as_str();
	}

	return workspace_feats_general(&content);
}

pub enum Active {
	ACTIVE,
	INACTIVE,
	NONE,
}

pub enum Rarity {
	COMMON,
	UNCOMMON,
	RARE,
	VERYRARE,
	LEGENDARY,
	ARTIFACT,
}
impl Rarity {
	pub fn to_string(&self) -> String {
		return match *self {
			Rarity::UNCOMMON => String::from("uncommon"),
			Rarity::RARE => String::from("rare"),
			Rarity::VERYRARE => String::from("veryrare"),
			Rarity::LEGENDARY => String::from("legendary"),
			Rarity::ARTIFACT => String::from("artifact"),
			_ => String::from("common"),
		};
	}
}

struct TestItem {
	active: Active,
	rarity: Rarity,
	name: String,
	tags: Vec<String>,
	weight: u64,
	quantity: u64,
	cost: f64,
	notes: String,
}

#[tauri::command]
pub fn player_inventory() -> String {
	let equipment_items: Vec<TestItem> = vec![
		TestItem {
			active: Active::ACTIVE,
			rarity: Rarity::UNCOMMON,
			name: String::from("Periapt of Health"),
			tags: vec![String::from("Wondrous item")],
			weight: 0,
			quantity: 1,
			cost: 0f64,
			notes: String::from("1 Charge"),
		},
		TestItem {
			active: Active::NONE,
			rarity: Rarity::RARE,
			name: String::from("Clothes, Common"),
			tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
			weight: 3,
			quantity: 1,
			cost: 0.5,
			notes: String::from("Social, Outerwear"),
		},
		TestItem {
			active: Active::NONE,
			rarity: Rarity::VERYRARE,
			name: String::from("Emblem"),
			tags: vec![String::from("Gear"), String::from("Holy Symbol")],
			weight: 0,
			quantity: 1,
			cost: 5f64,
			notes: String::from("Utility"),
		},
		TestItem {
			active: Active::NONE,
			rarity: Rarity::LEGENDARY,
			name: String::from("Orb"),
			tags: vec![String::from("Gear"), String::from("Arcane Focus")],
			weight: 3,
			quantity: 1,
			cost: 20f64,
			notes: String::from("Utility"),
		},
		TestItem {
			active: Active::ACTIVE,
			rarity: Rarity::ARTIFACT,
			name: String::from("Quaterstaff"),
			tags: vec![String::from("Quaterstaff")],
			weight: 4,
			quantity: 1,
			cost: 0.2,
			notes: String::from("Simple, Versatile, Topple"),
		},
		TestItem {
			active: Active::NONE,
			rarity: Rarity::COMMON,
			name: String::from("Spellbook"),
			tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
			weight: 3,
			quantity: 1,
			cost: 50f64,
			notes: String::from("Utility"),
		},
		TestItem {
			active: Active::NONE,
			rarity: Rarity::COMMON,
			name: String::from("Vestments"),
			tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
			weight: 0,
			quantity: 1,
			cost: 0f64,
			notes: String::from("Social, Utility"),
		},
	];

	let mut containers = HashMap::<String, Vec<TestItem>>::new();
	containers.insert(
		String::from("Backpack"),
		vec![
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Book"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 5,
				quantity: 1,
				cost: 25f64,
				notes: String::from("Social, Utility"),
			},
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Ink (1 ounce bottle)"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 0,
				quantity: 1,
				cost: 10f64,
				notes: String::from("Communication, Social, Utility, Consumable"),
			},
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Ink Pen"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 0,
				quantity: 1,
				cost: 0.02,
				notes: String::from("Communication, Social, Utility"),
			},
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Little Bag of Sand"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 0,
				quantity: 1,
				cost: 0f64,
				notes: String::from("Utility"),
			},
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Parchment (one sheet)"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 0,
				quantity: 10,
				cost: 1f64,
				notes: String::from("Communication, Social, Utility, Consumable"),
			},
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Small Knife"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 0,
				quantity: 1,
				cost: 0f64,
				notes: String::from("Utility"),
			},
		],
	);
	containers.insert(String::from("Bag of Holding"), vec![]);

	let attunement_items = vec![TestItem {
		active: Active::ACTIVE,
		rarity: Rarity::UNCOMMON,
		name: String::from("Periapt of Health"),
		tags: vec![String::from("Wondrous item")],
		weight: 0,
		quantity: 1,
		cost: 0f64,
		notes: String::from("1 Charge"),
	}];

	let other_possessions = vec![String::from("a prayer book"), String::from("5 sticks of incense")];

	let mut equipment_content = String::new();
	for item in &equipment_items {
		equipment_content += workspace_inventory_containeritem(
			&item.active,
			&item.rarity,
			&item.name,
			&item.tags,
			&item.weight,
			&item.quantity,
			&item.cost,
			&item.notes,
		)
		.as_str();
	}

	// TODO: make the ordering consistant (alphabetically or something)
	let mut containers_content = String::new();
	for (key, container) in &containers {
		let mut container_items = String::new();
		for item in container {
			container_items += workspace_inventory_containeritem(
				&item.active,
				&item.rarity,
				&item.name,
				&item.tags,
				&item.weight,
				&item.quantity,
				&item.cost,
				&item.notes,
			)
			.as_str();
		}
		containers_content += workspace_inventory_container(
			&Active::ACTIVE,
			&key,
			&(container.len() as u64),
			&(5 as u64),
			&container.iter().map(|i| i.weight as u64).sum(),
			&(30 as u64),
			&container_items,
		)
		.as_str();
	}

	let mut attunement_content = String::new();
	for item in &attunement_items {
		attunement_content += workspace_inventory_attunmentitem(&item.active, &item.rarity, &item.name, &item.tags).as_str();
	}

	return workspace_inventory(
		&(equipment_items.len() as u64),
		&equipment_items.iter().map(|i| i.weight as u64).sum(),
		&equipment_content,
		&containers_content,
		&attunement_items[0].rarity,
		&attunement_items[0].name,
		&attunement_items[0].tags,
		&Rarity::COMMON,
		&String::new(),
		&vec![],
		&Rarity::COMMON,
		&String::new(),
		&vec![],
		&attunement_content,
		&other_possessions,
	);
}

#[tauri::command]
pub fn player_inventory_equipment() -> String {
	let items: Vec<TestItem> = vec![
		TestItem {
			active: Active::ACTIVE,
			rarity: Rarity::UNCOMMON,
			name: String::from("Periapt of Health"),
			tags: vec![String::from("Wondrous item")],
			weight: 0,
			quantity: 1,
			cost: 0f64,
			notes: String::from("1 Charge"),
		},
		TestItem {
			active: Active::NONE,
			rarity: Rarity::RARE,
			name: String::from("Clothes, Common"),
			tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
			weight: 3,
			quantity: 1,
			cost: 0.5,
			notes: String::from("Social, Outerwear"),
		},
		TestItem {
			active: Active::NONE,
			rarity: Rarity::VERYRARE,
			name: String::from("Emblem"),
			tags: vec![String::from("Gear"), String::from("Holy Symbol")],
			weight: 0,
			quantity: 1,
			cost: 5f64,
			notes: String::from("Utility"),
		},
		TestItem {
			active: Active::NONE,
			rarity: Rarity::LEGENDARY,
			name: String::from("Orb"),
			tags: vec![String::from("Gear"), String::from("Arcane Focus")],
			weight: 3,
			quantity: 1,
			cost: 20f64,
			notes: String::from("Utility"),
		},
		TestItem {
			active: Active::ACTIVE,
			rarity: Rarity::ARTIFACT,
			name: String::from("Quaterstaff"),
			tags: vec![String::from("Quaterstaff")],
			weight: 4,
			quantity: 1,
			cost: 0.2,
			notes: String::from("Simple, Versatile, Topple"),
		},
		TestItem {
			active: Active::NONE,
			rarity: Rarity::COMMON,
			name: String::from("Spellbook"),
			tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
			weight: 3,
			quantity: 1,
			cost: 50f64,
			notes: String::from("Utility"),
		},
		TestItem {
			active: Active::NONE,
			rarity: Rarity::COMMON,
			name: String::from("Vestments"),
			tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
			weight: 0,
			quantity: 1,
			cost: 0f64,
			notes: String::from("Social, Utility"),
		},
	];

	let mut content = String::new();
	for item in &items {
		content += workspace_inventory_containeritem(
			&item.active,
			&item.rarity,
			&item.name,
			&item.tags,
			&item.weight,
			&item.quantity,
			&item.cost,
			&item.notes,
		)
		.as_str();
	}

	return workspace_inventory_equipment(&(items.len() as u64), &items.iter().map(|i| i.weight as u64).sum(), &content);
}

#[tauri::command]
pub fn player_inventory_containers(container_id: String) -> String {
	let mut containers = HashMap::<String, Vec<TestItem>>::new();
	containers.insert(
		String::from("Backpack"),
		vec![
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Book"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 5,
				quantity: 1,
				cost: 25f64,
				notes: String::from("Social, Utility"),
			},
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Ink (1 ounce bottle)"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 0,
				quantity: 1,
				cost: 10f64,
				notes: String::from("Communication, Social, Utility, Consumable"),
			},
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Ink Pen"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 0,
				quantity: 1,
				cost: 0.02,
				notes: String::from("Communication, Social, Utility"),
			},
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Little Bag of Sand"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 0,
				quantity: 1,
				cost: 0f64,
				notes: String::from("Utility"),
			},
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Parchment (one sheet)"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 0,
				quantity: 10,
				cost: 1f64,
				notes: String::from("Communication, Social, Utility, Consumable"),
			},
			TestItem {
				active: Active::NONE,
				rarity: Rarity::COMMON,
				name: String::from("Small Knife"),
				tags: vec![String::from("Legacy"), String::from("Gear"), String::from("Adventuring Gear")],
				weight: 0,
				quantity: 1,
				cost: 0f64,
				notes: String::from("Utility"),
			},
		],
	);
	containers.insert(String::from("Bag of Holding"), vec![]);

	let backup: Vec<TestItem> = vec![];
	let container = containers.get(&container_id).unwrap_or(&backup);

	let mut content = String::new();
	for item in container {
		content += workspace_inventory_containeritem(
			&item.active,
			&item.rarity,
			&item.name,
			&item.tags,
			&item.weight,
			&item.quantity,
			&item.cost,
			&item.notes,
		)
		.as_str();
	}

	return workspace_inventory_container(
		&Active::ACTIVE,
		&container_id,
		&(container.len() as u64),
		&(5 as u64),
		&container.iter().map(|i| i.weight as u64).sum(),
		&(30 as u64),
		&content,
	);
}

#[tauri::command]
pub fn player_inventory_containers_buttons() -> String {
	let containers = vec![String::from("Backpack"), String::from("Bag of Holding")];

	let mut content = String::new();
	for container in &containers {
		content += workspace_inventory_extratabbutton(&container, &container).as_str();
	}
	return content;
}

#[tauri::command]
pub fn player_inventory_attunement() -> String {
	let items = vec![TestItem {
		active: Active::ACTIVE,
		rarity: Rarity::UNCOMMON,
		name: String::from("Periapt of Health"),
		tags: vec![String::from("Wondrous item")],
		weight: 0,
		quantity: 1,
		cost: 0f64,
		notes: String::from("1 Charge"),
	}];
	let mut items_content = String::new();
	for item in &items {
		items_content += workspace_inventory_attunmentitem(&item.active, &item.rarity, &item.name, &item.tags).as_str();
	}
	return workspace_inventory_attunement(
		&items[0].rarity,
		&items[0].name,
		&items[0].tags,
		&Rarity::COMMON,
		&String::new(),
		&vec![],
		&Rarity::COMMON,
		&String::new(),
		&vec![],
		&items_content,
	);
}

#[tauri::command]
pub fn player_inventory_otherpossessions() -> String {
	let items = vec![String::from("a prayer book"), String::from("5 sticks of incense")];
	return workspace_inventory_otherpossessions(&items);
}

#[tauri::command]
pub fn player_inventory_currency() -> String {
	return 232.to_string();
}

#[tauri::command]
pub fn player_inventory_currency_platinum() -> String {
	return 20.to_string();
}

#[tauri::command]
pub fn player_inventory_currency_gold() -> String {
	return 20.to_string();
}

#[tauri::command]
pub fn player_inventory_currency_electrum() -> String {
	return 20.to_string();
}

#[tauri::command]
pub fn player_inventory_currency_silver() -> String {
	return 20.to_string();
}

#[tauri::command]
pub fn player_inventory_currency_copper() -> String {
	return 20.to_string();
}

#[tauri::command]
pub fn player_savingthrows_strength_bonus() -> String {
	return 0.to_string();
}
#[tauri::command]
pub fn player_savingthrows_dexterity_bonus() -> String {
	return 5.to_string();
}
#[tauri::command]
pub fn player_savingthrows_constitution_bonus() -> String {
	return 2.to_string();
}
#[tauri::command]
pub fn player_savingthrows_intelligence_bonus() -> String {
	return 10.to_string();
}
#[tauri::command]
pub fn player_savingthrows_wisdom_bonus() -> String {
	return 5.to_string();
}
#[tauri::command]
pub fn player_savingthrows_charisma_bonus() -> String {
	return 0.to_string();
}

#[tauri::command]
pub fn player_savingthrows_strength_status() -> String {
	return proficiency_untrained(&String::from("top"));
}
#[tauri::command]
pub fn player_savingthrows_dexterity_status() -> String {
	return proficiency_untrained(&String::from("top"));
}
#[tauri::command]
pub fn player_savingthrows_constitution_status() -> String {
	return proficiency_untrained(&String::from("top"));
}
#[tauri::command]
pub fn player_savingthrows_intelligence_status() -> String {
	return proficiency_proficient(&String::from("top"));
}
#[tauri::command]
pub fn player_savingthrows_wisdom_status() -> String {
	return proficiency_proficient(&String::from("top"));
}
#[tauri::command]
pub fn player_savingthrows_charisma_status() -> String {
	return proficiency_untrained(&String::from("top"));
}

#[tauri::command]
pub fn player_spells() -> String {
	return workspace_spells_table();
}
