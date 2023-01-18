use std::fs;

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

fn main() {
    let file_name: String = "ex.yml".to_string();
    let yml: String = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let docs = YamlLoader::load_from_str(&yml).unwrap();
    println!("{:?}", docs);
}
