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
