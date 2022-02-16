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
  pub sources: Option<Vec<Option<String>>>,
  pub sources_content: Option<Vec<Option<String>>>,
  pub names: Option<Vec<String>>,
  pub line_offset: Option<i64>,
  pub column_offset: Option<i64>,
}

fn convert_option_vec(s: &Option<Vec<Option<String>>>) -> Vec<&str> {
  s.as_ref().map_or(Vec::new(), |s| {
    s.iter()
      .map(|s| s.as_deref().unwrap_or_default())
      .collect::<Vec<_>>()
  })
}

#[napi]
impl SourceMap {
  #[napi(factory, ts_args_type = "vlqMaps: Array<String | VlqMap>")]
  pub fn merge_maps(vlq_jsons: Vec<String>) -> Result<Self> {
    let vlq_maps = vlq_jsons
      .par_iter()
      .map(|json| serde_json::from_str(json.as_str()).unwrap())
      .collect::<Vec<VlqMap>>();

    let speedy_vlq: Vec<SpeedyVlqMap> = vlq_maps
      .iter()
      .map(|map| {
        let sources = convert_option_vec(&map.sources);

        let sources_content = convert_option_vec(&map.sources_content);

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

  #[napi]
  pub fn set_source_content(&mut self, source_index: u32, source_content: String) -> Result<()> {
    Ok(
      self
        .0
        .set_source_content(source_index as usize, source_content.as_str())?,
    )
  }

  #[napi]
  pub fn get_source_content(&self, source_index: u32) -> Result<String> {
    Ok(self.0.get_source_content(source_index as usize)?.to_owned())
  }

  #[napi]
  pub fn to_url(&mut self) -> Result<String> {
    Ok(self.0.generate_url()?)
  }

  #[napi(ts_return_type = string)]
  pub fn to_comment(&self) -> Result<()> {
    // only for .d.ts declaration generation
    Ok(())
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
