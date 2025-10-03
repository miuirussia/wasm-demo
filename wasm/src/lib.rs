mod utils;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi,large_number_types_as_bigints,missing_as_null)]
#[serde(tag="type")]
pub struct Photo {
  #[serde(alias = "albumId")]
  album_id: i32,
  id: i64,
  title: String,
  url: String,
  #[serde(alias = "thumbnailUrl")]
  thumbnail_url: String,
}

#[wasm_bindgen]
pub fn parse(s: &str) -> Vec<Photo> {
  set_panic_hook();
  serde_json::from_str(s).unwrap()
}
