use napi::bindgen_prelude::*;
use napi::Result;
use napi_derive::napi;
use rayon::prelude::*;
use serde::Deserialize;

use speedy_sourcemap::{SourceMap as SpeedySourceMap, VlqMap as SpeedyVlqMap};

pub fn create_external<T>(value: T) -> External<T> {
  External::new(value)
}

#[napi]
#[derive(Clone)]
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
  /// Create Speedy SourceMap from external Sourcemap instance. It's useful when storing cache on Node.js side
  #[napi(factory)]
  pub fn new_from_external_sourcemap(external: External<&SpeedySourceMap>) -> Self {
    Self((*external).clone())
  }

  #[napi(factory, ts_args_type = "vlqMaps: Array<String | VlqMap>")]
  pub fn merge_maps(vlq_jsons: Vec<String>) -> Result<Self> {
    let vlq_maps = vlq_jsons
      .par_iter()
      .map(|json| serde_json::from_str(json.as_str()).unwrap())
      .collect::<Vec<VlqMap>>();

    let speedy_vlq: Vec<SpeedyVlqMap> = vlq_maps
      .iter()
      .map(|map| {
        let sources = map.sources.as_ref().map_or(Vec::new(), |s| {
          s.iter().map(|s| s.as_str()).collect::<Vec<_>>()
        });

        let sources_content = map.sources_content.as_ref().map_or(Vec::new(), |s| {
          s.iter().map(|s| s.as_str()).collect::<Vec<_>>()
        });

        let names = map.names.as_ref().map_or(Vec::new(), |s| {
          s.iter().map(|s| s.as_str()).collect::<Vec<_>>()
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
      speedy_vlq.iter().collect::<Vec<&SpeedyVlqMap>>().as_slice(),
    )?))
  }

  /// Convert Speedy SourceMap to External Value which can be stored in Node.js side indefinitely and useful when making mapChains or any caches
  #[napi]
  pub fn to_external_sourcemap(&self) -> External<&SpeedySourceMap> {
    create_external(&self.0)
  }

  #[napi]
  pub fn to_comment(&mut self) -> Result<String> {
    Ok(self.0.generate_comment()?)
  }

  #[napi]
  pub fn to_string(&mut self) -> Result<String> {
    Ok(self.0.generate_string()?)
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
