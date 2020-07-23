extern crate csv; 
extern crate serde_json;
mod utils;
//pub mod netget;
pub mod budget;
//pub mod canvas3d;

use serde::{ Deserialize, Serialize};
use serde_json::{Value};
use wasm_bindgen::prelude::*;
use web_sys::{console};
//use netget::net1;
use csv::{ReaderBuilder};
use js_sys::{Array, Uint8Array};
use budget::{rtn_budget, JsBudget, load_csv, fetch_csv, get_tree, JsNode, T1};
use budget::nodedata::*;
use std::panic;
use std::mem::drop;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(raw: JsValue) {
    //alert("Hello, t3!");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console::log_1(&raw);  // &"Test".into());
}

#[wasm_bindgen]
pub struct TreePtr {
    #[wasm_bindgen(skip)]
    pub nodes: Vec<Node>
}
#[wasm_bindgen]
pub async fn agreet(raw: JsValue) -> Result<TreePtr, JsValue>{
    //alert("Hello, t3!");
    let mut nodes: Vec<Node> = Vec::new();
    // panic::set_hook(Box::new(console_error_panic_hook::hook));
    console::log_1(&raw);  // &"Test".into());
    Ok(TreePtr { nodes: nodes })

}

#[wasm_bindgen]
pub struct AcctsPtr {
    //#[wasm_bindgen(skip)]
    //pub nodes: Vec<Node>,
    #[wasm_bindgen(skip)]
    pub accts: Vec<Acct>,
}

#[wasm_bindgen]
pub fn raw2accts(raw: JsValue) -> Result<JsValue, JsValue> {
    //let 
    //console::log_1(&raw);  // &"Test".into());
    let accts = budget::rdcsv::raw2acct(raw);

    let j = JsValue::from_serde(&accts).unwrap();
    // Ok(AcctsPtr {accts: accts})
    Ok(j)
}

#[wasm_bindgen]
pub fn gen_tree(jaccts: &JsValue, jflt: JsValue) -> Result<JsValue, JsValue> {
//pub fn init_app(bdgt: JsValue) -> Result<NodesPtr, JsValue> {
//pub fn init_app(bdgt: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(jaccts);  // &"Test".into());
    let rslt = jaccts.into_serde();
    let mut accts: Vec<Acct> =  match rslt {
        Ok(a) => { a},
        Err(err) => {
            //console::log_1(&JsValue::from_str("Parse budger json"));
            return Err(JsValue::from_str("Parse budget json"))
        },
    };
    console::log_1(&JsValue::from_f64(accts.len() as f64));  // &"Test".into());

    //: Vec<Acct> = jaccts.into_serde().unwrap();
    let fltr = budget::nodedata::Filter::new(); //jflt.into_serde().unwrap();
    let rtn = rtn_tree(accts, &fltr).unwrap();
    let j = JsValue::from_serde(&rtn).unwrap();
 
    Ok(j)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/*
#[wasm_bindgen]
pub fn jsnodes(nodes_ctx: &mut NodesPtr) -> Result<JsValue, JsValue> {
    let jstre: Vec<JsNode> = nodes_ctx.nodes.iter().map(|n| {JsNode::from_node(n)}).collect();
    let mut tre: Vec::<JsNode> = jstre;  
    Ok(JsValue::from_serde(&tre).unwrap())
}

#[wasm_bindgen]
pub fn getvals(nodes_ctx: &mut NodesPtr) -> Box<[f64]> {
    //let n = nodes_ctx.nodes.;
    nodes_ctx.nodes.iter().map(|n| {n.val as f64}).collect::<Vec<f64>>().into_boxed_slice()
}

#[wasm_bindgen]
pub fn chgval(nodes_ctx: &mut NodesPtr, idx:i32, dif: f64) -> Box<[f64]> {
    //let n = nodes_ctx.nodes.;
    let mut nodes = nodes_ctx.nodes.as_mut();

    let out = chg_node(nodes,idx, dif);
    nodes_ctx.nodes.iter().map(|n| {n.val as f64}).collect::<Vec<f64>>().into_boxed_slice()
}


#[wasm_bindgen]
pub fn free_nodes(nodes_ctx: &mut NodesPtr) {
    drop(nodes_ctx);
}



#[wasm_bindgen]
pub async fn net2(val: JsValue) -> Result<JsValue, JsValue> {
//    let rtn = netget::net1(val).await?;

//    let ary = Uint8Array::from(rtn);
//    let the: &[u8] = &ary.to_vec();
//    println!("{:?}",the);
//    let bdgt = rtn_budget(the).unwrap();
    let s = val.as_string().unwrap();
    let bdgt = load_csv(s).await?;

    Ok(bdgt)
}

#[wasm_bindgen]
pub async fn net3(bdgt: JsValue, filter: JsValue) -> Result<JsValue, JsValue> {
    let s: JsBudget = bdgt.into_serde().unwrap();
    console::log_1(&filter);
    let x: T1 = filter.into_serde().unwrap();
    //let f: String = filter.as_string().unwrap();
    let tre = get_tree(s);
    //console::log_1(&"Test tree".into());
    let jstre: Vec<JsNode> = tre.iter().map(|n| {JsNode::from_node(n)}).collect();

    let mut tre: Vec::<JsNode> = jstre;  
    Ok(JsValue::from_serde(&tre).unwrap())
}

#[wasm_bindgen]
pub fn tryit(s: JsValue) -> Result<JsValue, JsValue> {
    console::log_1(&s);  // &"Test".into());
    Ok(s)
}

#[wasm_bindgen]
pub fn wlog() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    //let tst = global();
    console::log_1(&"Test".into());
    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
*/