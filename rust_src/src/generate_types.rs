use serde_json::Value;
use typify::import_types;

// pub fn testing_types() {
//     import_types!(schema = "../data/schema/life.json");

//     let life = Life {
//         life_background: Value::String("red".to_string()),
//         life_class: Value::String("the life gamer".to_string()),
//         life_trinket: Value::String("locket".to_string()),
//     };

//     println!("{}", life.life_background);
// }

use std::{env, fs, path::Path};

use typify::{TypeSpace, TypeSpaceSettings};

pub fn build() {
    let content = std::fs::read_to_string("../data/schema/spells/spells.json").unwrap();
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&content).unwrap();

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(schema).unwrap();

    let contents =
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap());

    let mut out_file = Path::new("src/generated_structs").to_path_buf();
    out_file.push("spells.rs");
    fs::write(out_file, contents).unwrap();
}

// include!("./generated_structs/codegen.rs");

// pub fn testing_build() {
// 	let life = Life {
// 		life_background: Value::String("red".to_string()),
// 		life_class: Value::String("gamer".to_string()),
// 		life_trinket: Value::String("gaming".to_string())
// 	};

// 	println!("{:?}", life);
// }
