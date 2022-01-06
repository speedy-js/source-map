#[cfg(test)]
mod test {
  use speedy_sourcemap::{merge_map, SourceMap, VlqMap};

  #[test]
  fn should_work_with_fn() {
    // let foo = () => "foo";
    let babel_transformed = VlqMap {
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

    let minified = VlqMap {
      mappings: "AAAA,aAEA,IAAIA,IAAM,WACR,MAAO".as_bytes(),
      sources: vec!["0"],
      sources_content: vec![r#""use strict";\n\nvar foo = function foo() {\n  return "foo";\n};"#],
      names: vec!["foo"],
      line_offset: None,
      column_offset: None,
    };

    // "use strict";var foo=function(){return"foo"};

    let mut result = SourceMap::merge_maps(&[&minified, &babel_transformed]);
    assert!(result.is_ok());

    let mut vlq_output: Vec<u8> = vec![];
    assert!(result
      .as_mut()
      .unwrap()
      .inner
      .write_vlq(&mut vlq_output)
      .is_ok());

    let mappings = result.as_ref().unwrap().inner.get_mappings();

    assert_eq!(mappings[0].generated_column, 0);
    assert_eq!(mappings[0].generated_line, 0);
    assert_eq!(mappings[1].generated_column, 13);
    assert_eq!(mappings[1].original.unwrap().original_line, 0);
    assert_eq!(mappings[1].original.unwrap().original_column, 0);
    assert_eq!(mappings[1].original.unwrap().source, 1);
  }

  #[test]
  fn should_merge_maps_with_macros() {
    // let foo = () => "foo";
    let babel_transformed = VlqMap {
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

    let minified = VlqMap {
      mappings: "AAAA,aAEA,IAAIA,IAAM,WACR,MAAO".as_bytes(),
      sources: vec!["0"],
      sources_content: vec![r#""use strict";\n\nvar foo = function foo() {\n  return "foo";\n};"#],
      names: vec!["foo"],
      line_offset: None,
      column_offset: None,
    };

    // "use strict";var foo=function(){return"foo"};

    let mut result = merge_map!(&minified, &babel_transformed);
    assert!(result.is_ok());

    let mut vlq_output: Vec<u8> = vec![];
    assert!(result
      .as_mut()
      .unwrap()
      .inner
      .write_vlq(&mut vlq_output)
      .is_ok());

    let mappings = result.as_ref().unwrap().inner.get_mappings();

    assert_eq!(mappings[0].generated_column, 0);
    assert_eq!(mappings[0].generated_line, 0);
    assert_eq!(mappings[1].generated_column, 13);
    assert_eq!(mappings[1].original.unwrap().original_line, 0);
    assert_eq!(mappings[1].original.unwrap().original_column, 0);
    assert_eq!(mappings[1].original.unwrap().source, 1);
  }
}
