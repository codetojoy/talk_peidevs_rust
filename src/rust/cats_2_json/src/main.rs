
use serde_json;
use serde::{Deserialize, Serialize};
use std::fs;

use std::env;
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cat {
    pub name: String,
    pub age: u8,
}

pub fn build_from_json(config_file: &str) -> Vec::<Cat> {
    let data = fs::read_to_string(config_file).expect("Unable to read file");
    let cats: Vec::<Cat> = serde_json::from_str(&data).unwrap();
    return cats;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file = &args[1];

    let mut cats = build_from_json(config_file);

    cats.sort_by(|a, b| b.name.cmp(&a.name));

    for cat in cats {
        println!("TRACER {:?}", cat);
    }
    println!("TRACER Ready.");
}
