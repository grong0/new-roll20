// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;

// use generated_module::feats;

// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: feats = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

pub type Feats = Option<serde_json::Value>;
