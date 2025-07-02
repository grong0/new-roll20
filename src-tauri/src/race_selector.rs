use crate::{
    components::{badge_primary_md, race_category, race_collapse, race_dropdown},
    dao::DAO,
};

#[tauri::command]
pub fn race_categories() -> String {
    let dao = DAO::new();
    let mut content = String::new();
    for name in dao.races.get_unique_race_names() {
        content += race_category(&name).as_str();
    }
    return content;
}

#[tauri::command]
pub fn race_category_content(race_category_name: String) -> String {
    let dao = &DAO::new();
    let mut content = String::new();
    for race in dao.races.get_races_by_name(&race_category_name).iter() {
        if race.subraces.len() > 0 {
            content += race_dropdown(&race.name, &race.source.name).as_str();
        } else {
            let book_name = match dao.books.get_book_from_source(&race.source.name) {
                Some(book) => book.name.clone(),
                None => race.source.name.clone(),
            };
            let npc_extension = if race.trait_tags.contains(&"NPC Race".to_string()) { badge_primary_md(&"NPC Race".to_string()) } else { "".to_string() };
            content += race_collapse(&format!("{} {}", &race.name, &npc_extension), &book_name).as_str();
        }
    }
    return content;
}

// #[tauri::command]
// pub fn race_details(race: String) -> String {

// }


