mod utils;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi,large_number_types_as_bigints,missing_as_null)]
pub struct Point {
  x: i64,
  y: i64,
}

#[wasm_bindgen]
pub fn parse(s: &str) -> Point {
  set_panic_hook();
  serde_json::from_str::<Point>(s).unwrap()
}
