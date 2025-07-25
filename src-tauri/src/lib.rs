mod components;
// mod dao;
mod frontend_functions;
mod layout2;
mod serde_utils;

use crate::layout2::*;

#[tauri::command]
fn gamer() -> String {
	return "we be gaming!!".to_string();
}

#[tauri::command]
fn fortnite() -> String {
	return "fortnite!!!".to_string();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			player_name,
			player_race,
			player_level,
			player_classes,
			player_hitpoints_max,
			player_hitpoints_current,
			player_hitpoints_percent,
			player_hitpoints_temp,
			player_armorclass,
			player_initiative,
			player_speed_walking,
			player_class_level,
			player_abilities_str_score,
			player_abilities_dex_score,
			player_abilities_con_score,
			player_abilities_int_score,
			player_abilities_wis_score,
			player_abilities_cha_score,
			player_abilities_str_modifier,
			player_abilities_dex_modifier,
			player_abilities_con_modifier,
			player_abilities_int_modifier,
			player_abilities_wis_modifier,
			player_abilities_cha_modifier,
			player_skills_acrobatics_status,
			player_skills_animalhandling_status,
			player_skills_arcana_status,
			player_skills_athletics_status,
			player_skills_deception_status,
			player_skills_history_status,
			player_skills_insight_status,
			player_skills_intimidation_status,
			player_skills_investigation_status,
			player_skills_medicine_status,
			player_skills_nature_status,
			player_skills_perception_status,
			player_skills_performance_status,
			player_skills_persuasion_status,
			player_skills_religion_status,
			player_skills_slightofhand_status,
			player_skills_stealth_status,
			player_skills_survival_status,
			player_skills_acrobatics_bonus,
			player_skills_animalhandling_bonus,
			player_skills_arcana_bonus,
			player_skills_athletics_bonus,
			player_skills_deception_bonus,
			player_skills_history_bonus,
			player_skills_insight_bonus,
			player_skills_intimidation_bonus,
			player_skills_investigation_bonus,
			player_skills_medicine_bonus,
			player_skills_nature_bonus,
			player_skills_perception_bonus,
			player_skills_performance_bonus,
			player_skills_persuasion_bonus,
			player_skills_religion_bonus,
			player_skills_slightofhand_bonus,
			player_skills_stealth_bonus,
			player_skills_survival_bonus,
			player_senses_perception,
			player_senses_investigation,
			player_senses_insight,
			player_proficiencies_armor,
			player_proficiencies_weapons,
			player_proficiencies_tools,
			player_proficiencies_languages,
			player_actions,
			player_actions_action,
			player_actions_bonusaction,
			player_actions_reaction,
			player_actions_other,
			player_actions_limiteduse,
			player_actionsincombat_action,
			player_actionsincombat_bonusaction,
			player_actionsincombat_reaction,
			player_actionsincombat_other,
			player_actionsincombat_limiteduse,
			player_inventory,
			player_inventory_equipment,
			player_inventory_attunement,
			player_inventory_other,
			player_inventory_currency,
			player_inventory_currency_platinum,
			player_inventory_currency_gold,
			player_inventory_currency_electrum,
			player_inventory_currency_silver,
			player_inventory_currency_copper,
			player_feats,
			player_feats_class,
			player_feats_race,
			player_feats_general,
			gamer,
			fortnite
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
