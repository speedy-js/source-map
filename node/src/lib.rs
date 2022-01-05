#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use napi::Result;
use rayon::prelude::*;
use serde::Deserialize;

use speedy_sourcemap::{SourceMap as SpeedySourceMap, VlqMap as SpeedyVlqMap};

pub fn create_external<T>(value: T) -> External<T> {
  External::new(value)
}

#[napi]
pub struct SourceMap(SpeedySourceMap);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[napi(object)]
pub struct VlqMap {
  pub mappings: String,
  pub sources: Option<Vec<String>>,
  pub sources_content: Option<Vec<String>>,
  pub names: Option<Vec<String>>,
  pub line_offset: Option<i64>,
  pub column_offset: Option<i64>,
}

#[napi]
impl SourceMap {
  #[napi(factory, ts_args_type = "vlqMaps: Array<String | VlqMap>")]
  pub fn merge_maps(vlq_jsons: Vec<String>) -> Result<Self> {
    let vlq_maps = vlq_jsons
      .par_iter()
      .map(|json| serde_json::from_str(json.as_str()).unwrap())
      .collect::<Vec<VlqMap>>();

    let mut speedy_vlq = vlq_maps
      .par_iter()
      .map(|map| {
        let sources = map.sources.as_ref().map_or(Vec::new(), |s| {
          s.par_iter().map(|s| s.as_str()).collect::<Vec<_>>()
        });

        let sources_content = map.sources_content.as_ref().map_or(Vec::new(), |s| {
          s.par_iter().map(|s| s.as_str()).collect::<Vec<_>>()
        });

        let names = map.names.as_ref().map_or(Vec::new(), |s| {
          s.par_iter().map(|s| s.as_str()).collect::<Vec<_>>()
        });

        SpeedyVlqMap {
          mappings: map.mappings.as_bytes(),
          sources,
          sources_content,
          names,
          line_offset: None,
          column_offset: None,
        }
      })
      .collect::<Vec<SpeedyVlqMap>>();

    Ok(SourceMap(SpeedySourceMap::merge_maps(
      &mut speedy_vlq.iter_mut().collect::<Vec<_>>(),
    )?))
  }

  #[napi]
  pub fn to_comment(&mut self) -> Result<String> {
    Ok(self.0.to_comment()?)
  }

  #[napi]
  pub fn to_string(&mut self) -> Result<String> {
    Ok(self.0.to_string()?)
  }

  #[napi(ts_return_type = "{
  version: number
  mappings: string
  names: string[]
  sources: (string | null)[]
  sourcesContent: (string | null)[]
  file?: string | null
  sourceRoot?: string | null
  }")]
  pub fn to_map(&mut self) -> Result<()> {
    // only for .d.ts generation
    Ok(())
  }
}
