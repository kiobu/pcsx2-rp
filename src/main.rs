use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{PathBuf};
use std::env;
use serde_json::{Value};

pub fn read_config() -> Value {
    let cdir = match env::current_dir() {
        Ok(pathbuf) => { pathbuf },
        Err(_) => panic!("No buffer.")
    };

    let path = cdir.join(PathBuf::from("config.json"));

    println!("{:?}", &path);

    let file = File::open(path)
        .expect("Could not open file.");
    let mut buffer = BufReader::new(file);
    let mut result = String::new();

    let _: usize = match buffer.read_to_string(&mut result) {
        Ok(_n) => _n,
        Err(_) => 0
    };

    return match serde_json::from_str(&result[0..]) {
        Ok(obj) => { obj },
        Err(_) => panic!("Could not parse JSON.")
    };
}

fn main() {
    let path = &read_config()["path"];

    println!("{}", path);
}
