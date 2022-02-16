const fs = require('fs')
const path = require('path')
const { SourceMap } = require('../node')

const transformedMap = fs.readFileSync(
  path.resolve(__dirname, './fixtures/antd/antd.js.map'),
  'utf-8',
)
const minifiedMap = fs.readFileSync(
  path.resolve(__dirname, './fixtures/antd/antd.min.js.map'),
  'utf-8',
)

for (let i = 0; i < 3; i++) {
  const mergeMaps = SourceMap.mergeMaps([minifiedMap, transformedMap])
  mergeMaps.toComment()
}
