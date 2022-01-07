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
    const map = sourcemap.toMap()

    assert.equal(map.version, 3)

    assert.ok(sourcemap.toComment().startsWith("//# sourceMappingURL="))
    assert.ok(sourcemap.toUrl().startsWith("data:application/json;charset=utf-8;base64,"))
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

