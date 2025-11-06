use std::fs::read_to_string;

use crate::layout2::{Active, Rarity};

// TODO: for whole file: use tauri resources instead of normal pathing

pub fn proficiency_untrained(tooptip_direction: &String) -> String {
	let file =
		read_to_string("components/proficiency/untrained.html").unwrap_or("".to_string()).replace("{tooltip_direction}", tooptip_direction);
	return file;
}

pub fn proficiency_halfproficient(tooptip_direction: &String) -> String {
	let file = read_to_string("components/proficiency/halfproficient.html")
		.unwrap_or(String::new())
		.replace("{tooltip_direction}", tooptip_direction);
	return file;
}

pub fn proficiency_proficient(tooptip_direction: &String) -> String {
	let file = read_to_string("components/proficiency/proficient.html")
		.unwrap_or("".to_string())
		.replace("{tooltip_direction}", tooptip_direction);
	return file;
}

pub fn proficiency_expertise(tooptip_direction: &String) -> String {
	let file =
		read_to_string("components/proficiency/expertise.html").unwrap_or("".to_string()).replace("{tooltip_direction}", tooptip_direction);
	return file;
}

pub fn workspace_actions_item(
	name: &String,
	action_type: &String,
	range: &String,
	hitdc: &String,
	damage: &String,
	damage_type: &String,
	notes: &String,
) -> String {
	let file = read_to_string("components/workspace/actions/item.html")
		.unwrap_or("".to_string())
		.replace("{name}", name)
		.replace("{type}", action_type)
		.replace("{range}", range)
		.replace("{hitdc}", hitdc)
		.replace("{damage}", damage)
		.replace("{damage_type}", damage_type)
		.replace("{notes}", notes);
	return file;
}

pub fn workspace_actions(
	actions: &String,
	bonus_actions: &String,
	reactions: &String,
	other_actions: &String,
	limited_use_actions: &String,
	actions_in_combat: &String,
	bonus_actions_in_combat: &String,
	reactions_in_combat: &String,
	other_actions_in_combat: &String,
	limited_use_actions_in_combat: &String,
) -> String {
	let file = read_to_string("components/workspace/actions/allactions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{bonus_actions}", bonus_actions)
		.replace("{reactions}", reactions)
		.replace("{other_actions}", other_actions)
		.replace("{limited_use_actions}", limited_use_actions)
		.replace("{actions_in_combat}", actions_in_combat)
		.replace("{bonus_actions_in_combat}", bonus_actions_in_combat)
		.replace("{reactions_in_combat}", reactions_in_combat)
		.replace("{other_actions_in_combat}", other_actions_in_combat)
		.replace("{limited_use_actions_in_combat}", limited_use_actions_in_combat);
	return file;
}

pub fn workspace_actions_action(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("components/workspace/actions/actions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_bonusaction(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("components/workspace/actions/bonusactions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_reaction(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("components/workspace/actions/reactions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_other(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("components/workspace/actions/otheractions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_limiteduse(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("components/workspace/actions/limiteduseactions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn class_badge(class_higher: &String, class_lower: &String) -> String {
	let file = read_to_string("components/class-badge.html")
		.unwrap_or("".to_string())
		.replace("{class}", class_higher)
		.replace("{class_lower}", class_lower);
	return file;
}

pub fn workspace_feats(class_content: &String, racial_content: &String, general_content: &String) -> String {
	let file = read_to_string("components/workspace/feats/allfeats.html")
		.unwrap_or(String::new())
		.replace("{class_content}", class_content)
		.replace("{racial_content}", racial_content)
		.replace("{general_content}", general_content);
	return file;
}

pub fn workspace_feats_class(content: &String) -> String {
	let file = read_to_string("components/workspace/feats/classfeats.html").unwrap_or(String::new()).replace("{content}", content);
	return file;
}

pub fn workspace_feats_race(content: &String) -> String {
	let file = read_to_string("components/workspace/feats/racialfeats.html").unwrap_or(String::new()).replace("{content}", content);
	return file;
}

pub fn workspace_feats_general(content: &String) -> String {
	let file = read_to_string("components/workspace/feats/generalfeats.html").unwrap_or(String::new()).replace("{content}", content);
	return file;
}

pub fn workspace_feats_item(name: &String, source: &String, page: &u64, content: &String) -> String {
	let file = read_to_string("components/workspace/feats/item.html")
		.unwrap_or(String::new())
		.replace("{name}", name)
		.replace("{source}", source)
		.replace("{page}", page.to_string().as_str())
		.replace("{content}", content);
	return file;
}

pub fn workspace_feats_classheader(name: &String) -> String {
	let file = read_to_string("components/workspace/feats/classheader.html").unwrap_or(String::new()).replace("{name}", name);
	return file;
}

pub fn checkbox(size: &str, checked: &bool) -> String {
	let file = read_to_string("components/checkbox.html")
		.unwrap_or(String::new())
		.replace("{size}", size)
		.replace("{checked}", &checked.to_string());
	return file;
}

pub fn workspace_inventory(
	num_of_equipment_items: &u64,
	equipment_weight: &u64,
	equipment_items: &String,
	containers: &String,
	attunement_slot1_rarity: &Rarity,
	attunement_slot1_name: &String,
	attunement_slot1_tags: &Vec<String>,
	attunement_slot2_rarity: &Rarity,
	attunement_slot2_name: &String,
	attunement_slot2_tags: &Vec<String>,
	attunement_slot3_rarity: &Rarity,
	attunement_slot3_name: &String,
	attunement_slot3_tags: &Vec<String>,
	attunement_items: &String,
	other_possessions: &Vec<String>,
) -> String {
	let file = read_to_string("components/workspace/inventory/allinventory.html")
		.unwrap_or(String::new())
		.replace("{num_of_equipment_items}", num_of_equipment_items.to_string().as_str())
		.replace("{equipment_weight}", equipment_weight.to_string().as_str())
		.replace("{equipment_items}", equipment_items)
		.replace("{containers}", containers)
		.replace("{attunement_slot1_rarity}", &attunement_slot1_rarity.to_string())
		.replace(
			"{attunement_slot1_name}",
			if attunement_slot1_name.len() == 0 { "<p class='text-base-content/60'>Empty</p>" } else { attunement_slot1_name },
		)
		.replace("{attunement_slot1_tags}", &attunement_slot1_tags.join(" • "))
		.replace("{attunement_slot2_rarity}", &attunement_slot2_rarity.to_string())
		.replace(
			"{attunement_slot2_name}",
			if attunement_slot2_name.len() == 0 { "<p class='text-base-content/60'>Empty</p>" } else { attunement_slot2_name },
		)
		.replace("{attunement_slot2_tags}", &attunement_slot2_tags.join(" • "))
		.replace("{attunement_slot3_rarity}", &attunement_slot3_rarity.to_string())
		.replace(
			"{attunement_slot3_name}",
			if attunement_slot3_name.len() == 0 { "<p class='text-base-content/60'>Empty</p>" } else { attunement_slot3_name },
		)
		.replace("{attunement_slot3_tags}", &attunement_slot3_tags.join(" • "))
		.replace("{attunement_items}", attunement_items)
		.replace("{other_possessions}", &other_possessions.iter().map(|i| format!("<p>{}</p>", i)).collect::<Vec<String>>().join("\n"));
	return file;
}

pub fn workspace_inventory_equipment(num_of_items: &u64, weight: &u64, content: &String) -> String {
	let file = read_to_string("components/workspace/inventory/equipment.html")
		.unwrap_or(String::new())
		.replace("{num_of_items}", num_of_items.to_string().as_str())
		.replace("{weight}", weight.to_string().as_str())
		.replace("{content}", content);
	return file;
}

pub fn workspace_inventory_container(
	active: &Active,
	name: &String,
	num_of_items: &u64,
	container_weight: &u64,
	contains_weight: &u64,
	max_weight: &u64,
	content: &String,
) -> String {
	let active_element = match &active {
		Active::ACTIVE => checkbox("xs", &true),
		Active::INACTIVE => checkbox("xs", &false),
		Active::NONE => String::new(),
	};
	let file = read_to_string("components/workspace/inventory/container.html")
		.unwrap_or(String::new())
		.replace("{active}", &active_element)
		.replace("{weight_constraints}", format!("({}/{} lb.)", contains_weight.to_string(), max_weight.to_string()).as_str())
		.replace("{name}", name)
		.replace("{num_of_items}", num_of_items.to_string().as_str())
		.replace("{weight}", (container_weight + contains_weight).to_string().as_str())
		.replace("{content}", content);
	return file;
}

pub fn workspace_inventory_containeritem(
	active: &Active,
	rarity: &Rarity,
	name: &String,
	tags: &Vec<String>,
	weight: &u64,
	quantity: &u64,
	cost: &f64,
	notes: &String,
) -> String {
	// TODO: make active into a component
	let active_element = match &active {
		Active::ACTIVE => checkbox("xs", &true),
		Active::INACTIVE => checkbox("xs", &false),
		Active::NONE => String::from("-"),
	};
	let tag_content = tags.join(" • ");
	let file = read_to_string("components/workspace/inventory/containeritem.html")
		.unwrap_or(String::new())
		.replace("{active}", &active_element)
		.replace("{rarity}", rarity.to_string().as_str())
		.replace("{name}", name)
		.replace("{tags}", &tag_content)
		.replace("{weight}", weight.to_string().as_str())
		.replace("{quantity}", quantity.to_string().as_str())
		.replace("{cost}", cost.to_string().as_str())
		.replace("{notes}", notes);
	return file;
}

pub fn workspace_inventory_extratabbutton(label: &String, container_id: &String) -> String {
	let file = read_to_string("components/workspace/inventory/extratabbutton.html")
		.unwrap_or(String::new())
		.replace("{label}", label)
		.replace("{container_id}", container_id);
	return file;
}

pub fn workspace_inventory_attunement(
	slot1_rarity: &Rarity,
	slot1_name: &String,
	slot1_tags: &Vec<String>,
	slot2_rarity: &Rarity,
	slot2_name: &String,
	slot2_tags: &Vec<String>,
	slot3_rarity: &Rarity,
	slot3_name: &String,
	slot3_tags: &Vec<String>,
	items: &String,
) -> String {
	// TODO: make empty <p> a component
	let file = read_to_string("components/workspace/inventory/attunement.html")
		.unwrap_or(String::new())
		.replace("{slot1_rarity}", &slot1_rarity.to_string())
		.replace("{slot1_name}", if slot1_name.len() == 0 { "<p class='text-base-content/60'>Empty</p>" } else { slot1_name })
		.replace("{slot1_tags}", &slot1_tags.join(" • "))
		.replace("{slot2_rarity}", &slot2_rarity.to_string())
		.replace("{slot2_name}", if slot2_name.len() == 0 { "<p class='text-base-content/60'>Empty</p>" } else { slot2_name })
		.replace("{slot2_tags}", &slot2_tags.join(" • "))
		.replace("{slot3_rarity}", &slot3_rarity.to_string())
		.replace("{slot3_name}", if slot3_name.len() == 0 { "<p class='text-base-content/60'>Empty</p>" } else { slot3_name })
		.replace("{slot3_tags}", &slot3_tags.join(" • "))
		.replace("{items}", &items);
	return file;
}

pub fn workspace_inventory_attunmentitem(active: &Active, rarity: &Rarity, name: &String, tags: &Vec<String>) -> String {
	// TODO: make active into a component
	let active_element = match &active {
		Active::ACTIVE => checkbox("xs", &true),
		Active::INACTIVE => checkbox("xs", &false),
		Active::NONE => String::from("-"),
	};
	let tag_content = tags.join(" • ");
	let file = read_to_string("components/workspace/inventory/attunementitem.html")
		.unwrap_or(String::new())
		.replace("{active}", &active_element)
		.replace("{rarity}", &rarity.to_string())
		.replace("{name}", name)
		.replace("{tags}", &tag_content);
	return file;
}

pub fn workspace_inventory_otherpossessions(items: &Vec<String>) -> String {
	let file = read_to_string("components/workspace/inventory/otherpossessions.html")
		.unwrap_or(String::new())
		.replace("{items}", &items.iter().map(|i| format!("<p>{}</p>", i)).collect::<Vec<String>>().join("\n"));
	return file;
}

pub fn workspace_spells_table(content: &String) -> String {
	let file = read_to_string("components/workspace/spells/table.html").unwrap_or(String::new()).replace("{content}", content);
	return file;
}

// 20 ft.
// <span
// 	><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 18 18.06" class="h-4 inline-block">
// 		<path fill="#b0b7bd" d="M9,1A8,8,0,1,1,1,9,8,8,0,0,1,9,1M9,0a9,9,0,1,0,9,9A9,9,0,0,0,9,0Z"></path>
// 		<path
// 			fill="#b0b7bd"
// 			d="M9,18.06a.5.5,0,0,1,0-1c2,0,3.65-3.68,3.65-8S11,1,9,1A.5.5,0,0,1,9,0c2.61,0,4.65,4,4.65,9S11.61,18.06,9,18.06Z"
// 		></path>
// 		<path
// 			fill="#b0b7bd"
// 			d="M9.48,11.44A18.11,18.11,0,0,1,.28,8.84.5.5,0,0,1,.78,8c9,5.25,16.37.49,16.44.44a.5.5,0,0,1,.56.83A16.25,16.25,0,0,1,9.48,11.44Z"
// 		></path>
// 	</svg> </span
// >, V/S/M

pub fn workspace_spells_item(
	name: &String,
	source: &String,
	extra_icons: &String,
	entries: &String,
	time_amount: &String,
	time_unit_full: &String,
	time_unit_abbreviated: &String,
	range: &String,
	range_unit: &String,
	save_unit: &String,
	save_dc: &String,
	damage_die: &String,
	components: &String,
	duration_amount: &String,
	duration_unit_full: &String,
	duration_unit_abbreviated: &String,
	notes: &String,
) -> String {
	let file = read_to_string("components/workspace/spells/item.html")
		.unwrap_or(String::new())
		.replace("{name}", name)
		.replace("{source}", source)
		.replace("{extra_icons}", extra_icons)
		.replace("{entries}", entries)
		.replace("{time_amount}", time_amount)
		.replace("{time_unit_full}", time_unit_full)
		.replace("{time_unit_abbreviated}", time_unit_abbreviated)
		.replace("{range}", range)
		.replace("{range_unit}", range_unit)
		.replace("{save_unit}", save_unit)
		.replace("{save_dc}", save_dc)
		.replace("{damage_die}", damage_die)
		.replace("{components}", components)
		.replace("{duration_amount}", duration_amount)
		.replace("{duration_unit_full}", duration_unit_full)
		.replace("{duration_unit_abbreviated}", duration_unit_abbreviated)
		.replace("{notes}", notes);
	return file;
}

// TODO: look into changing svgs to prefer either w or h based on their shape
pub fn get_svg(path: &str, width: &str, height: &str) -> String {
	return read_to_string(format!("components/svgs/{}.svg", path))
		.unwrap_or(String::new())
		.replace("{width}", width)
		.replace("{height}", height);
}

pub fn tooltip(content: &String, tooltip: &String, direction: &String) -> String {
	return read_to_string(format!("components/tooltip.html"))
		.unwrap_or(String::new())
		.replace("{content}", content)
		.replace("{tooltip}", tooltip)
		.replace("{direction}", direction);
}
