extern crate serde_json;

use std::io::prelude::*;
use std::io;
use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use serde_json::{Value, ser};
use std::process;

fn main() {
    let filepath    = Path::new(&env::home_dir().unwrap()).join(".gifs");
    let mut fread   = OpenOptions::new().read(true).open(&filepath).unwrap();
    let mut content = String::new();

    fread.read_to_string(&mut content);

    let mut data: Value = serde_json::from_str(&content).unwrap();
    let mut obj         = data.as_object_mut().unwrap();

    let count = env::args().count();
    if count < 2 {
        println!("Usage: {} key value", env::args().next().unwrap());
        process::exit(1);
    }

    let key = env::args().skip(1).next().unwrap();

    let mut value = String::new();
    if count == 3 {
        value = env::args().skip(2).next().unwrap();
    } else if count == 2 {
        io::stdin().read_to_string(&mut value);
        value = value.trim_right().to_string();
    };

    obj.insert(key, Value::String(value));

    let mut fwrite = OpenOptions::new().write(true).truncate(true).open(filepath).unwrap();
    fwrite.write(ser::to_string(obj).unwrap().as_bytes());
}
