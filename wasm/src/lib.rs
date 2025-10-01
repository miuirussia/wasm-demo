mod utils;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi)]
pub struct Point {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn parse(s: &str) -> Point {
    set_panic_hook();
    serde_json::from_str(s).unwrap()
}
