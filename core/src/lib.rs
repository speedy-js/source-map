#![deny(clippy::all)]

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

#[derive(Debug, Clone)]
pub struct Vlq {
  pub mappings: String,
  pub names: Vec<String>,
  pub sources: Vec<String>,
  pub sources_content: Vec<String>,
}

impl SourceMap {
  /// Create a new Speedy SourceMap instance directly from Parcel SourceMap
  pub fn new(parcel_sourcemap: PSourceMap) -> Self {
    Self {
      inner: parcel_sourcemap,
      vlq: None,
    }
  }

  /// Create a new Speedy SourceMap instance from Parcel SourceMap buffer
  pub fn new_from_buffer(buf: &[u8]) -> Result<Self> {
    Ok(Self {
      inner: PSourceMap::from_buffer("/", buf)?,
      vlq: None,
    })
  }

  /// Merge SourceMaps from given vlq mappings
  pub fn merge_maps(vlq_maps: &[&VlqMap]) -> Result<Self> {
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

  pub fn generate_vlq(&mut self) -> Result<&Vlq> {
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

  pub fn generate_map(&mut self) -> Result<RawSourceMap> {
    let vlq_map = self.generate_vlq()?;
    Ok(RawSourceMap::new_from_vlq(vlq_map))
  }

  pub fn generate_comment(&mut self) -> Result<String> {
    let raw_map = self.generate_map()?;
    raw_map.to_url()
  }

  pub fn generate_string(&mut self) -> Result<String> {
    let raw_map = self.generate_map()?;
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

          SourceMap::merge_maps(vlq_maps.as_slice())
      }
  };
}
