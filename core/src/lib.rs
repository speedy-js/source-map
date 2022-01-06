#![deny(clippy::all)]

#[cfg(feature = "node-api")]
use napi_derive::napi;

use parcel_sourcemap::SourceMap as PSourceMap;
use rayon::prelude::*;

mod raw_sourcemap;
mod result;

pub use raw_sourcemap::RawSourceMap;
pub use result::*;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SourceMap {
  pub inner: PSourceMap,
  vlq: Option<Vlq>,
}

#[derive(Debug)]
pub struct VlqMap<'a> {
  pub mappings: &'a [u8],
  pub sources: Vec<&'a str>,
  pub sources_content: Vec<&'a str>,
  pub names: Vec<&'a str>,
  pub line_offset: Option<i64>,
  pub column_offset: Option<i64>,
}

#[cfg(feature = "node-api")]
#[napi(object)]
#[derive(Debug, Clone)]
pub struct Vlq {
  pub mappings: String,
  pub names: Vec<String>,
  pub sources: Vec<String>,
  pub sources_content: Vec<String>,
}

#[cfg(not(feature = "node-api"))]
#[derive(Debug, Clone)]
pub struct Vlq {
  pub mappings: String,
  pub names: Vec<String>,
  pub sources: Vec<String>,
  pub sources_content: Vec<String>,
}

impl SourceMap {
  pub fn new(parcel_sourcemap: PSourceMap) -> Self {
    Self {
      inner: parcel_sourcemap,
      vlq: None,
    }
  }

  pub fn new_from_buffer(buf: &[u8]) -> Result<Self> {
    Ok(Self {
      inner: PSourceMap::from_buffer("/", buf)?,
      vlq: None,
    })
  }

  pub fn merge_maps(vlq_maps: &mut [&mut VlqMap]) -> Result<Self> {
    let len = vlq_maps.len();
    assert!(len > 0);

    let mut parcel_sm: Vec<Option<PSourceMap>> = vlq_maps
      .into_par_iter()
      .map(|vlq_map| {
        let mut sm = PSourceMap::new("");
        sm.add_vlq_map(
          vlq_map.mappings,
          vlq_map.sources.to_vec(),
          vlq_map.sources_content.to_vec(),
          vlq_map.names.to_vec(),
          vlq_map.line_offset.unwrap_or(0),
          vlq_map.column_offset.unwrap_or(0),
        )?;
        Ok(Some(sm))
      })
      .collect::<Result<Vec<_>>>()?;

    if len == 1 {
      return Ok(SourceMap::new(parcel_sm[0].take().unwrap()));
    };

    let last = parcel_sm.last_mut().unwrap().take().unwrap();
    let (source_maps, _) = parcel_sm.split_at_mut(len - 1);

    let parcel_sourcemap = source_maps.iter_mut().try_rfold::<PSourceMap, fn(
      PSourceMap,
      &mut Option<PSourceMap>,
    ) -> Result<PSourceMap>, Result<PSourceMap>>(
      last,
      |mut prev_map, map| {
        map.as_mut().unwrap().extends(&mut prev_map)?;
        Ok(map.take().unwrap())
      },
    )?;

    Ok(Self::new(parcel_sourcemap))
  }

  pub fn to_vlq(&mut self) -> Result<&Vlq> {
    let mut vlq_output: Vec<u8> = vec![];
    self.inner.write_vlq(&mut vlq_output)?;

    self.vlq = Some(Vlq {
      mappings: String::from_utf8(vlq_output)?,
      names: self.inner.get_names().clone(),
      sources: self.inner.get_sources().clone(),
      sources_content: self.inner.get_sources_content().clone(),
    });

    Ok(self.vlq.as_ref().unwrap())
  }

  pub fn to_map(&mut self) -> Result<RawSourceMap> {
    let vlq_map = self.to_vlq()?;
    Ok(RawSourceMap::new_from_vlq(vlq_map))
  }

  pub fn to_comment(&mut self) -> Result<String> {
    let raw_map = self.to_map()?;
    raw_map.to_url()
  }

  pub fn to_string(&mut self) -> Result<String> {
    let raw_map = self.to_map()?;
    raw_map.to_string()
  }
}

#[macro_export]
macro_rules! merge_map {
  ($( $vlq_map: expr), *) => {
      {
          let mut vlq_maps = vec![];

          $(
            vlq_maps.push($vlq_map);
          )*

          SourceMap::merge_maps(vlq_maps.as_mut_slice())
      }
  };
}

#[test]
fn should_merge_map() {
  // let foo = () => "foo";
  let mut babel_transformed = VlqMap {
    mappings: ";;AAAA,IAAIA,GAAG,GAAG,SAANA,GAAM;AAAA,SAAM,KAAN;AAAA,CAAV".as_bytes(),
    sources: vec!["unknown"],
    sources_content: vec![r#"let foo = () => "foo";"#],
    names: vec!["foo"],
    line_offset: None,
    column_offset: None,
  };

  // "use strict";
  //
  // var foo = function foo() {
  //   return "foo";
  // };

  let mut minified = VlqMap {
    mappings: "AAAA,aAEA,IAAIA,IAAM,WACR,MAAO".as_bytes(),
    sources: vec!["0"],
    sources_content: vec![r#""use strict";\n\nvar foo = function foo() {\n  return "foo";\n};"#],
    names: vec!["foo"],
    line_offset: None,
    column_offset: None,
  };
  // "use strict";var foo=function(){return"foo"};

  let mut result = merge_map!(&mut minified, &mut babel_transformed)
    .unwrap()
    .inner;

  let mut vlq_output: Vec<u8> = vec![];
  assert!(result.write_vlq(&mut vlq_output).is_ok());

  let mappings = result.get_mappings();

  assert_eq!(mappings[0].generated_column, 0);
  assert_eq!(mappings[0].generated_line, 0);
  assert_eq!(mappings[1].generated_column, 13);
  assert_eq!(mappings[1].original.unwrap().original_line, 0);
  assert_eq!(mappings[1].original.unwrap().original_column, 0);
  assert_eq!(mappings[1].original.unwrap().source, 1);
}
