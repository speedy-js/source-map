use serde::Serialize;

use crate::result::Result;
use crate::Vlq;

static VERSION: u8 = 3;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawSourceMap {
  pub version: u8,
  pub mappings: String,
  pub names: Vec<String>,
  #[serde(default)]
  pub sources: Vec<Option<String>>,
  pub sources_content: Vec<Option<String>>,
  pub file: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(default)]
  pub source_root: Option<String>,
}

impl RawSourceMap {
  pub fn new(
    mappings: &str,
    file: Option<&str>,
    names: Vec<&str>,
    sources_content: Vec<Option<&str>>,
    source_root: Option<&str>,
    sources: Vec<Option<&str>>,
  ) -> Self {
    Self {
      version: VERSION,
      mappings: String::from(mappings),
      file: file.map(|f| f.to_owned()),
      names: names.iter().map(|&n| n.to_owned()).collect::<Vec<String>>(),
      sources_content: sources_content
        .iter()
        .map(|s| s.map(|s| s.to_owned()))
        .collect(),
      source_root: source_root.map(|s| s.to_owned()),
      sources: sources.iter().map(|s| s.map(|s| s.to_owned())).collect(),
    }
  }

  pub fn new_from_vlq(vlq: &Vlq) -> Self {
    Self {
      version: VERSION,
      mappings: vlq.mappings.clone(),
      names: vlq.names.iter().map(|s| s.to_owned()).collect::<Vec<_>>(),
      sources: vlq
        .sources
        .iter()
        .map(|source| Some(source.to_owned()))
        .collect::<Vec<_>>(),
      sources_content: vlq
        .sources_content
        .iter()
        .map(|sc| Some(sc.to_owned()))
        .collect::<Vec<_>>(),
      file: None,
      source_root: None,
    }
  }

  /// ## Generate SourceMap in JSON format
  pub fn to_string(&self) -> Result<String> {
    Ok(serde_json::to_string(self)?)
  }

  /// ## Generate inline SourceMap
  pub fn to_url(&self) -> Result<String> {
    let str = Self::to_string(self)?;

    Ok(format!(
      "data:application/json;charset=utf-8;base64,{}",
      base64::encode(str)
    ))
  }
}
