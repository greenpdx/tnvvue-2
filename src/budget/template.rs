use serde::{ Deserialize, Serialize};
use std::error::Error;
use wasm_bindgen::prelude::*;
use js_sys::JsString;
use std::f64;


use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response}; // TextDecoder};
use web_sys::console;
use js_sys::{Uint8Array, Array};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TmplHeader {

}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TmplAcct {
    acode: i16,
    bcode: i16,
    ccode: i32,
    val: f64,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TNVTemplate {
    head: TmplHeader,
    accts: Vec<TmplAcct>,
}

