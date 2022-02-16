import assert from 'assert'
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

    assert.ok(sourcemap.toComment().startsWith('//# sourceMappingURL='))
    assert.ok(
      sourcemap
        .toUrl()
        .startsWith('data:application/json;charset=utf-8;base64,'),
    )
    assert.deepEqual(map, {
      version: 3,
      mappings: 'A,aCAA,IAAIA,IAAM,WAAA,MAAM',
      names: ['foo'],
      sources: ['0', 'unknown'],
      sourcesContent: [
        'use strict";\n\nvar foo = function foo() {\n  return "foo";\n};',
        'let foo = () => "foo";',
      ],
      file: null,
    })
  })

  it('should support string maps', () => {
    const sourcemap = SourceMap.mergeMaps([
      JSON.stringify({
        mappings: 'AAAA,aAEA,IAAIA,IAAM,WACR,MAAO',
        sources: ['0'],
        sourcesContent: [
          `use strict";\n\nvar foo = function foo() {\n  return "foo";\n};`,
        ],
        names: ['foo'],
      }),
      JSON.stringify({
        mappings: ';;AAAA,IAAIA,GAAG,GAAG,SAANA,GAAM;AAAA,SAAM,KAAN;AAAA,CAAV',
        sources: ['unknown'],
        sourcesContent: [`let foo = () => "foo";`],
        names: ['foo'],
      }),
    ])
    const map = sourcemap.toMap()

    assert.equal(map.version, 3)

    assert.ok(sourcemap.toComment().startsWith('//# sourceMappingURL='))
    assert.ok(
      sourcemap
        .toUrl()
        .startsWith('data:application/json;charset=utf-8;base64,'),
    )
    assert.deepEqual(map, {
      version: 3,
      mappings: 'A,aCAA,IAAIA,IAAM,WAAA,MAAM',
      names: ['foo'],
      sources: ['0', 'unknown'],
      sourcesContent: [
        'use strict";\n\nvar foo = function foo() {\n  return "foo";\n};',
        'let foo = () => "foo";',
      ],
      file: null,
    })
  })
})
