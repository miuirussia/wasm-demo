mod utils;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Serializer;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Tsify, Serialize, Deserialize)]
pub struct Point {
  #[tsify(type = "bigint")]
  x: i64,
  #[tsify(type = "bigint")]
  y: i64,
}

#[wasm_bindgen(unchecked_return_type = "Point")]
pub fn parse(s: &str) -> JsValue {
  set_panic_hook();
  (&serde_json::from_str::<Point>(s).unwrap()).serialize(&Serializer::new().serialize_large_number_types_as_bigints(true)).unwrap()
}
