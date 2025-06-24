mod components;
mod dao;
mod frontend_functions;
mod layout2;
mod serde_utils;

use crate::layout2::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            player_name,
            player_race,
            player_level,
            player_hitpoints_max,
            player_hitpoints_current,
            player_hitpoints_percent,
            player_hitpoints_temp,
            player_armorclass,
            player_initiative,
            player_speed_walking,
            player_class_level,
            player_attribute_score,
            player_attribute_modifier,
            player_skill_bonus,
            player_senses_perception,
            player_senses_investigation,
            player_senses_insight,
            player_proficiencies_armor,
            player_proficiencies_weapons,
            player_proficiencies_tools,
            player_proficiencies_languages
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
