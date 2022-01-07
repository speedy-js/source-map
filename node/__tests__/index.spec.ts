import assert from "assert";
import { SourceMap } from '..'

describe('merge', () => {
  it('should merge', () => {
    const sourcemap = SourceMap.mergeMaps([
      {
        mappings: 'AAAA,aAEA,IAAIA,IAAM,WACR,MAAO',
        sources: ['0'],
        sourcesContent: [
          `use strict";\n\nvar foo = function foo() {\n  return "foo";\n};`,
        ],
        names: ['foo'],
      },
      {
        mappings: ';;AAAA,IAAIA,GAAG,GAAG,SAANA,GAAM;AAAA,SAAM,KAAN;AAAA,CAAV',
        sources: ['unknown'],
        sourcesContent: [`let foo = () => "foo";`],
        names: ['foo'],
      },
    ])
    console.log(sourcemap.toComment())
    console.log(sourcemap.toString())
    console.log(sourcemap.toMap())
  })
})

describe('external', () => {
  it('should create and use external sourcemap', () => {
    const sourcemap = SourceMap.mergeMaps([
      {
        mappings: 'AAAA,aAEA,IAAIA,IAAM,WACR,MAAO',
        sources: ['0'],
        sourcesContent: [
          `use strict";\n\nvar foo = function foo() {\n  return "foo";\n};`,
        ],
        names: ['foo'],
      },
      {
        mappings: ';;AAAA,IAAIA,GAAG,GAAG,SAANA,GAAM;AAAA,SAAM,KAAN;AAAA,CAAV',
        sources: ['unknown'],
        sourcesContent: [`let foo = () => "foo";`],
        names: ['foo'],
      },
    ])
    const external = sourcemap.toExternalSourcemap()
    const sourcemap_external = SourceMap.newFromExternalSourcemap(external)

    assert.equal(sourcemap.toString(), sourcemap_external.toString())
  })
})

