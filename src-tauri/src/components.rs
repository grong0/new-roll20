use std::fs::read_to_string;

pub fn skills_status_untrained() -> String {
	let file = read_to_string("../components/skills/untrained.html").unwrap_or("".to_string());
	return file;
}

pub fn skills_status_proficient() -> String {
	let file = read_to_string("../components/skills/proficient.html").unwrap_or("".to_string());
	return file;
}

pub fn skills_status_expert() -> String {
	let file = read_to_string("../components/skills/expert.html").unwrap_or("".to_string());
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
	let file = read_to_string("../components/workspace/actions/item.html")
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
	let file = read_to_string("../components/workspace/actions/allactions.html")
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
	let file = read_to_string("../components/workspace/actions/actions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_bonusaction(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("../components/workspace/actions/bonusactions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_reaction(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("../components/workspace/actions/reactions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_other(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("../components/workspace/actions/otheractions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn workspace_actions_limiteduse(actions: &String, actions_in_combat: &String) -> String {
	let file = read_to_string("../components/workspace/actions/limiteduseactions.html")
		.unwrap_or("".to_string())
		.replace("{actions}", actions)
		.replace("{actions_in_combat}", actions_in_combat);
	return file;
}

pub fn class_badge(class_higher: &String, class_lower: &String) -> String {
	let file = read_to_string("../components/class-badge.html")
		.unwrap_or("".to_string())
		.replace("{class}", class_higher)
		.replace("{class_lower}", class_lower);
	return file;
}

pub fn workspace_feats(class_content: &String, racial_content: &String, general_content: &String) -> String {
	let file = read_to_string("../components/workspace/feats/allfeats.html")
		.unwrap_or(String::new())
		.replace("{class_content}", class_content)
		.replace("{racial_content}", racial_content)
		.replace("{general_content}", general_content);
	return file;
}

pub fn workspace_feats_class(content: &String) -> String {
	let file = read_to_string("../components/workspace/feats/classfeats.html").unwrap_or(String::new()).replace("{content}", content);
	return file;
}

pub fn workspace_feats_race(content: &String) -> String {
	let file = read_to_string("../components/workspace/feats/racialfeats.html").unwrap_or(String::new()).replace("{content}", content);
	return file;
}

pub fn workspace_feats_general(content: &String) -> String {
	let file = read_to_string("../components/workspace/feats/generalfeats.html").unwrap_or(String::new()).replace("{content}", content);
	return file;
}

pub fn workspace_feats_item(name: &String, source: &String, page: &u64, content: &String) -> String {
	let file = read_to_string("../components/workspace/feats/item.html")
		.unwrap_or(String::new())
		.replace("{name}", name)
		.replace("{source}", source)
		.replace("{page}", page.to_string().as_str())
		.replace("{content}", content);
	return file;
}

pub fn workspace_feats_classheader(name: &String) -> String {
	let file = read_to_string("../components/workspace/feats/classheader.html").unwrap_or(String::new()).replace("{name}", name);
	return file;
}

pub fn workspace_inventory(active: &bool, name: &String, num_of_items: &u64, weight: &u64, content: &String) -> String {
	let active_element = if *active { String::from("<input type='checkbox' class='checkbox' checked />") } else { String::new() };
	let file = read_to_string("../components/workspace/inventory/allinventory.html")
		.unwrap_or(String::new())
		.replace("{active}", &active_element)
		.replace("{name}", name)
		.replace("{num_of_items}", num_of_items.to_string().as_str())
		.replace("{weight}", weight.to_string().as_str())
		.replace("{content}", content);
	return file;
}
