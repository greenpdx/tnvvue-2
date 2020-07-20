use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::path::Path;
use wasm_bindgen::prelude::*;
use serde_json::{Result, Value};

mod budget;
use budget::*; 

fn main() {
    let path = Path::new("./test/data/tst.json");

    //let mut file = File::open("./test/data/tst.json").unwrap();
    let mut file = File::open("./public/mongodb").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf);
    let s = str::from_utf8(&buf).unwrap();
    //let j = JsValue::from_str(s);
    let j: Result<Value> = serde_json::from_str(s);
    //console::log_1(&bdgt);
    let accts = rdcsv::raw2acct(j.unwrap());
    println!("{:?}", &accts);
}

