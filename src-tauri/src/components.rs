use std::fs::read_to_string;

pub fn title_card_component() -> String {
	let file = read_to_string("../src/components/title-card.html").unwrap_or("".to_string());
	println!("the file's contents are: {}", file);
	return file;
}
