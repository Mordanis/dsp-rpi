use serde_json;

use std::fs::File;
use std::io::prelude::*;

#[derive(serde::Deserialize, Debug)]
struct Kernel {
    data: Vec<f32>
}

pub fn read_corrector() -> Vec<f32> {
    let mut file = File::open("corrector.json").unwrap();
    let mut tmp = String::new();
    let _contents = file.read_to_string(&mut tmp).unwrap();
    let out_data: Kernel = serde_json::from_str(&tmp).unwrap();
    out_data.data
}
