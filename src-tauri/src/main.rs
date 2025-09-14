// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod serde_utils;
mod version_checking;

use version_checking::{is_newer_version, update_data};

const UPDATE_DATA: bool = false;

fn main() {
	if UPDATE_DATA && is_newer_version() {
		let result = update_data();
		if result.is_err() {
			println!("{:?}", result.err());
		};
	}
	println!();

	new_roll20_lib::run()
}
