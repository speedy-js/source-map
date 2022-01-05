#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::Result;

use speedy_sourcemap::SourceMap as SpeedySourceMap;

pub fn create_external<T>(value: T) -> External<T> {
  External::new(value)
}

#[napi]
pub struct SourceMap(SpeedySourceMap);

#[napi]
impl SourceMap {
  #[napi]
  fn merge_maps() {}
}
