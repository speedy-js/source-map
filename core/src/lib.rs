#![deny(clippy::all)]

use rayon::prelude::*;

use parcel_sourcemap::SourceMap as PSourceMap;

struct SourceMap {}

struct VlqMap<'a> {
  input: &'a [u8],
  sources: Vec<&'static str>,
  sources_content: Vec<&'static str>,
  names: Vec<&'static str>,
  line_offset: Option<i64>,
  column_offset: Option<i64>,
}

fn merge_maps(vlq_maps: &mut [&mut VlqMap]) -> PSourceMap {
  let len = vlq_maps.len();
  assert!(len > 0);

  let mut parcel_sm = vlq_maps
    .into_par_iter()
    .map(|vlq_map| {
      let mut sm = PSourceMap::new("");
      sm.add_vlq_map(
        &vlq_map.input,
        vlq_map.sources.clone(),
        vlq_map.sources_content.clone(),
        vlq_map.names.clone(),
        match vlq_map.line_offset {
          Some(line_offset) => line_offset,
          None => 0,
        },
        match vlq_map.column_offset {
          Some(column_offset) => column_offset,
          None => 0,
        },
      );
      sm
    })
    .collect::<Vec<_>>();

  if len == 1 {
    return parcel_sm[0].clone();
  };

  let last = parcel_sm.last_mut().unwrap().clone();
  let (source_maps, _) = parcel_sm.split_at_mut(len - 2);

  source_maps
    .into_iter()
    .rfold(last, |mut prev_map, mut map| {
      map.extends(&mut prev_map);
      map.clone()
    })
}

macro_rules! merge_map {
  ($( $vlq_map: expr), *) => {
      {
          let mut vlq_maps = vec![];

          $(
            vlq_maps.push($vlq_map);
          )*

          merge_maps(vlq_maps.as_mut_slice())
      }
  };
}

#[test]
fn should_merge_map() {
  // let foo = () => "foo";
  let mut babel_transformed = VlqMap {
    input: ";;AAAA,IAAIA,GAAG,GAAG,SAANA,GAAM;AAAA,SAAM,KAAN;AAAA,CAAV".as_bytes(),
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
    input: "AAAA,aAEA,IAAIA,IAAM,WACR,MAAO".as_bytes(),
    sources: vec!["0"],
    sources_content: vec![r#""use strict";\n\nvar foo = function foo() {\n  return "foo";\n};"#],
    names: vec!["foo"],
    line_offset: None,
    column_offset: None,
  };
  // "use strict";var foo=function(){return"foo"};

  let mut result = merge_map!(&mut minified, &mut babel_transformed);

  let mut vlq_output: Vec<u8> = vec![];
  result.write_vlq(&mut vlq_output);

  println!(
    "mappings: {}, sources: {:#?}, sources_content: {:#?}, names: {:#?}",
    String::from_utf8(vlq_output).expect(""),
    result.get_sources(),
    result.get_sources_content(),
    result.get_names()
  );
}
