extern crate napi;
#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::Result;

pub fn create_external<T>(value: T) -> External<T> {
  External::new(value)
}

