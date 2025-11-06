use std::{collections::HashMap, fs::read_to_string};

use serde_json::{from_str, to_value, Map, Value};

use crate::serde_utils::{
	serde_as_array, serde_as_array_mapping, serde_as_object, serde_as_object_from_option, serde_as_string,
};

use super::common::{form_key, File};

#[derive(Debug)]
struct Ordinal {
	ordinal_type: String,
	identifier: String,
}

impl Ordinal {
	pub fn new(value: Option<&Value>) -> Ordinal {
		let object = serde_as_object_from_option(value, Map::new());

		return Ordinal {
			ordinal_type: serde_as_string(object.get("type"), "N/A".to_string()),
			identifier: serde_as_string(object.get("identifier"), "0".to_string()),
		};
	}
}

#[derive(Debug)]
struct Content {
	name: String,
	headers: Vec<String>,
	ordinal: Ordinal,
}

impl Content {
	pub fn new(value: Option<&Value>) -> Content {
		let object = serde_as_object_from_option(value, Map::new());

		return Content {
			name: serde_as_string(object.get("name"), "N/A".to_string()),
			headers: serde_as_array_mapping(object.get("headers"), serde_as_string, "N/A".to_string()),
			ordinal: Ordinal::new(object.get("ordinal")),
		};
	}
}

#[derive(Debug)]
pub struct Book {
	pub name: String,
	pub source: String,
	pub key: String,
	pub id: String,
	pub group: String,
	pub cover: File,
	pub published: String,
	pub author: String,
	pub contents: Vec<Content>,
	pub alias: Vec<String>,
}

impl Book {
	pub fn new(value: Value) -> Book {
		let object = serde_as_object(&value, Map::new());

		let name = serde_as_string(object.get("name"), "N/A".to_string());
		let source = serde_as_string(object.get("source"), "N/A".to_string());

		return Book {
			key: form_key(&name, &source),
			name,
			source,
			id: serde_as_string(object.get("id"), "N/A".to_string()),
			group: serde_as_string(object.get("group"), "N/A".to_string()),
			cover: File::new(object.get("cover")),
			published: serde_as_string(object.get("published"), "N/A".to_string()),
			author: serde_as_string(object.get("author"), "N/A".to_string()),
			contents: serde_as_array(object.get("contents")).iter().map(|i| Content::new(Some(i))).collect(),
			alias: serde_as_array_mapping(object.get("alias"), serde_as_string, "N/A".to_string()),
		};
	}
}

#[derive(Debug)]
pub struct Books {
	map: HashMap<String, Book>,
}

impl Books {
	pub fn new(path: &str) -> Books {
		let file = read_to_string(path);
		let map = HashMap::new();
		if file.is_err() {
			return Books { map };
		}
		let serde_file: Value = from_str(file.unwrap().as_str()).unwrap_or(to_value(Map::new()).unwrap());
		let value_list: Vec<Value> = serde_as_array(serde_file.get("book"));
		println!("num of books: {}", value_list.len());

		let mut num_of_na = 0;
		let mut map: HashMap<String, Book> = HashMap::new();
		for value in value_list {
			let new_struct = Book::new(value);
			if !new_struct.key.contains("n/a") {
				map.insert(new_struct.key.as_str().to_string(), new_struct);
			} else {
				num_of_na += 1;
			}
		}
		println!("number of books with no name: {}", num_of_na);

		return Books { map };
	}

	pub fn get_book_from_source(&self, source: &String) -> Option<&Book> {
		for (_, book) in self.map.iter() {
			if book.source.to_lowercase() == source.to_lowercase() {
				return Some(book);
			}
		}
		return None;
	}
}
