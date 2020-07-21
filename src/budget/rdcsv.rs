use serde::{ Deserialize};
use std::error::Error;
use std::path::Path;
use serde_json::{Result, Value};
use wasm_bindgen::prelude::*;

//use csv::{ReaderBuilder};

use super::nodedata::{BEACat, LKV, Budget, Acct, BKey};
//use ndarray_csv::{ArrayReader, ArrayWriter};
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
struct Record {
    agencycode: i32,
    agencyname: String,
    bureaucode: i32,
    bureauname: String,
    accountcode: Option<i32>,
    accountname: String,
    treasuryagencycode: Option<i16>,
    subfunctioncode: i16,
    subfunctiontitle: String,
    beacat: String,
    onoffbudget: String,
    y1976: i64,	
    tq: i64,
    y1977: i64,
    y1978: i64,
    y1979: i64,
    y1980: i64,
    y1981: i64,
    y1982: i64,
    y1983: i64,
    y1984: i64,
    y1985: i64,
    y1986: i64,
    y1987: i64,
    y1988: i64,
    y1989: i64,
    y1990: i64,
    y1991: i64,
    y1992: i64,
    y1993: i64,
    y1994: i64,
    y1995: i64,
    y1996: i64,
    y1997: i64,
    y1998: i64,
    y1999: i64,
    y2000: i64,
    y2001: i64,
    y2002: i64,
    y2003: i64,
    y2004: i64,
    y2005: i64,
    y2006: i64,
    y2007: i64,
    y2008: i64,
    y2009: i64,
    y2010: i64,
    y2011: i64,
    y2012: i64,
    y2013: i64,
    y2014: i64,
    y2015: i64,
    y2016: i64,
    y2017: i64,
    y2018: i64,
    y2019: i64,
    y2020: i64,
    y2021: i64,
    y2022: i64,
    y2023: i64,
    y2024: i64,
    idx: Option<i16>
}

pub fn raw2acct(raw: JsValue) -> Vec<Acct> {
    let z: Vec<Record> = raw.into_serde().unwrap();         // serde_json::from_value(raw).unwrap();
    let mut n: Vec<Acct> = Vec::new(); 
    for (i,t) in z.iter().enumerate() {
        let l = newleaf(t,i as i32);
        //println!("{:?}", l);
        n.push(l);
    }
    n
}

fn newleaf(rec: &Record, idx:i32) -> Acct {
    let val: Vec<i64> = vec![
rec.y1976, rec.tq, rec.y1977, rec.y1978, rec.y1979, rec.y1980, rec.y1981,
rec.y1982, rec.y1983, rec.y1984, rec.y1985, rec.y1986, rec.y1987,
rec.y1988, rec.y1989, rec.y1990, rec.y1991, rec.y1992, rec.y1993,
rec.y1994, rec.y1995, rec.y1996, rec.y1997, rec.y1998, rec.y1999,
rec.y2000, rec.y2001, rec.y2002, rec.y2003, rec.y2004, rec.y2005,
rec.y2006, rec.y2007, rec.y2008, rec.y2009, rec.y2010, rec.y2011,
rec.y2012, rec.y2013, rec.y2014, rec.y2015, rec.y2016, rec.y2017,
rec.y2018, rec.y2019, rec.y2020, rec.y2021, rec.y2022, rec.y2023,
rec.y2024
    ];
    //let val: Vec<i64> = Vec::new();
    // let val = val.iter().map(|v| {v * 1000000}).collect();
    let ccode = match rec.accountcode {
        Some(v) => v,
        None => i32::min_value()
    };
    let acct = Acct {
        idx: idx,
        key: BKey::new(rec.agencycode as i16, rec.bureaucode as i16, ccode),
        
        name: rec.accountname.clone(),
        tac: match rec.treasuryagencycode {
            Some(v) => v,
            None => i16::min_value()
        },
        scode: rec.subfunctioncode,
        bea: match rec.beacat.as_ref() {
            "N" => {BEACat::I},
            "M" => {BEACat::M},
            "D" => {BEACat::D},
            _ => {println!("Bad BEACat"); BEACat::X}
        },
        onoff: match rec.onoffbudget.as_ref() {
            "On-budget" => { true },
            "Off-budget" => { false },
            _ => { false }
        },
        astr: rec.agencyname.clone(),
        bstr: rec.bureauname.clone(),
        sub: rec.subfunctiontitle.clone(),
        value: val 
    };
    acct
}

fn idx_find_or_insert(vec: &mut Vec<LKV>, val: String, idx: i32) -> i32 {
    let kv = LKV::new(idx, val);
    if let Some(i) = (0..vec.len()).find(|&i| vec[i] == kv ) {
        i as i32
    } else {
        vec.push(kv);
        (vec.len() - 1)  as i32
    }
}

pub fn rtn_budget(data: String) -> Budget {
//pub fn rtn_budget(data: &[u8]) -> Budget {
//pub fn rtn_budget(data: &[u8]) -> Result<Budget, Box<dyn Error>> {
    //let b = data.as_bytes();
    let d  = data.as_str();
    let mut cnt: i16 = 0;  //d.len() as i16;
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(d.as_bytes());
 
 //   let reader = ReaderBuilder::new().has_headers(true).from_path(path).unwrap();
    let mut anames: Vec<LKV> = Vec::new();
    let mut bnames: Vec<LKV> = Vec::new();
    let mut snames: Vec<LKV> = Vec::new();
    let mut accts: Vec<Acct> = Vec::new();
    
    //println!("{:?}", hdr.len());
    //let (idx, rslt) = reader.deserialize().enumerate() {
    let iter = reader.deserialize();
    for (idx, rslt) in iter.enumerate() {
        let record: Record = rslt.unwrap();
        cnt = cnt + 1;
        //println!("{:?}", record);
        //let a = idx_find_or_insert(anames.as_mut(), record.agencyname.clone(), record.agencycode);
        //let b = idx_find_or_insert(bnames.as_mut(), record.bureauname.clone(), record.bureaucode);
        //let s = idx_find_or_insert(snames.as_mut(), record.subfunctiontitle.clone(), record.subfunctioncode as i32);
        //println!("{:?} {:?}", a, record);
        let acct = newleaf(&record, idx as i32);
 
        accts.push(acct);
    }
    let zot = reader.is_done();

    cnt = accts.len() as i16;
   println!("{:?}", accts);
    let budget = Budget { /*anames: anames, bnames: bnames, sname: snames,*/ accts: accts};
    //Ok(budget)
    //budget
    budget
}
