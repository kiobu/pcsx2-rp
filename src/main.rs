use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{PathBuf};
use std::env;
use serde_json::{Value};

fn read_config() -> Value {
    let cdir = match env::current_dir() {
        Ok(pathbuf) => { pathbuf },
        Err(_) => panic!("No buffer.")
    };

    let path = cdir.join(PathBuf::from("config.json"));

    let file = File::open(path)
        .expect("Could not open config.");
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

fn read_file(npath: String) -> String {

    let path = npath.replace("/", "\\");

    println!("{}", path);

    let file = File::open(PathBuf::from(String::from(path.trim())))
        .expect("Could not open file.");
    let mut buffer = BufReader::new(file);
    let mut result = String::new();

    let _: usize = match buffer.read_to_string(&mut result) {
        Ok(_n) => _n,
        Err(_) => 0
    };

    return result;
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let path = &read_config()["path"];

    if path == "" {
        println!(
        "Your path has not been set up! Make sure the path to your emuLog.txt is set in the config.json file. It should look like this: {}",
        "
        {
            \"path\": \"C:/my/path/emuLog.txt\"   
        }
        "
        );
        pause();
    } else {
        let result = read_file(path.to_string());
    }
}
