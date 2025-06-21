mod dao;
mod update_data;

use dao::DAO;

fn main() {
    // generate_types::build();
    // generate_types::testing_build();
    // let file = fs::read_dir("data");
    // if file.is_ok() {
    // 	println!("{:?}", file);
    // } else {
    // 	println!("no such file or directory")
    // }

    // update_data::update_data();
	let dao = DAO::new();

    println!("{:#?}", dao);
}
