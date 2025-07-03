use std::{
	fs::{create_dir, read_dir, read_to_string, remove_dir_all},
	iter::zip,
};

use serde_json::{from_str, Map, Value};

use crate::serde_utils::{serde_as_object_from_option, serde_as_string};

const REPO_URL: &str = "https://github.com/5etools-mirror-3/5etools-2014-src";

/// returns 1 if version 1 is newer, 2 if version 2 is newer, and 0 if they're the same
fn compare_versions(ver1: String, ver2: String) -> u8 {
	let split_ver1 = ver1.split(".").map(|num| num.parse::<u8>().unwrap());
	let split_ver2 = ver2.split(".").map(|num| num.parse::<u8>().unwrap());

	for (sub_ver_1, sub_ver_2) in zip(split_ver1, split_ver2) {
		if sub_ver_1 > sub_ver_2 {
			return 1;
		} else if sub_ver_2 > sub_ver_1 {
			return 2;
		}
		continue;
	}

	return 0;
}

fn get_version(path: &str) -> String {
	let file = read_to_string(path);

	if file.is_err() {
		println!("Failed to read file at '{}'", path);
		return "0.0.0".to_string();
	}

	let json: Value = from_str(file.unwrap().as_str()).unwrap();
	let array = json.as_array().unwrap();

	return serde_as_string(serde_as_object_from_option(array.get(array.len() - 1), Map::new()).get("ver"), "0.0.0".to_string());
}

pub fn is_newer_version() -> bool {
	// get local version
	let local_version = get_version("data/raw/changelog.json");

	// get remote version
	if create_dir("data/.temp").is_err() {
		println!("Failed to create temp folder at data/.temp");
		return false;
	}
	let downloader = git_download::repo(REPO_URL).branch_name("main").add_file("data/*", "data/.temp").exec();
	if downloader.is_err() {
		println!("{:?}", downloader.err().unwrap());
		return false;
	}
	let remote_version = get_version("data/.temp/changelog.json");
	if remove_dir_all("data/.temp").is_err() {
		println!("{:?}", "Failed to remove data/.temp folder");
		return false;
	}

	println!("local version: {}", local_version);
	println!("remote version: {}", remote_version);

	if compare_versions(local_version, remote_version) == 2 {
		return true;
	}
	return false;
}

pub fn update_data() -> Result<(), String> {
	if remove_dir_all("data/raw").is_err() {
		println!("Failed to delete current raw folder at data/raw");
	}

	if create_dir("data/raw").is_err() {
		return Err("Failed to create data/raw folder".to_string());
	}
	let result = git_download::repo(REPO_URL).branch_name("main").add_file("data/*", "data/raw").exec();
	if result.is_err() {
		return Err("Failed to download from remote repository.".to_string());
	}

	if read_dir("data/raw").is_err() {
		return Err("Failed to read new data folder at expected location (data/raw)".to_string());
	}

	return Ok(());
}
