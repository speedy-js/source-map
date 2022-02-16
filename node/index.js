const FastJSON = require('fast-json')
const { SourceMap } = require('./binding')

const ensureAndStoreSourcesContent = (map, idContentMap) => {
  if (typeof map === 'object') {
    const { sources = [], sourcesContent = [] } = map

    sources.forEach((id, i) => {
      idContentMap.set(id, sourcesContent[i])
      sourcesContent[i] = null
    })

    return JSON.stringify(map)
  }

  const fastJSON = new FastJSON()
  let mappingsString = null
  let sourcesString = '[]'
  let namesString = '[]'

  let sourcesContentCount = 0

  fastJSON.on('mappings', (value) => {
    mappingsString = value
  })
  fastJSON.on('sources', (value) => {
    sourcesString = value
  })
  fastJSON.on('sourcesContent', (value) => {
    sourcesContentCount += 1
    const sources = JSON.parse(sourcesString)
    const sourcesContent = JSON.parse(value)
    sources.forEach((id, i) => {
      idContentMap.set(id, sourcesContent[i])
    })
  })
  fastJSON.on('names', (value) => {
    namesString = value
  })
  fastJSON.write(map)

  const jsonString = `{"mappings":"${mappingsString}","sources":${sourcesString},"sourcesContent":[${new Array(
    sourcesContentCount,
  )
    .fill('null')
    .join(',')}],"names":${namesString},"version":3}`
  return jsonString
}

const bringBackSourcesContent = (map, idContentMap) => {
  const { sources = [] } = map
  const sourcesContent = sources.length
    ? new Array(sources.length).fill(null)
    : []

  sources.forEach((id, i) => {
    const sourceContent = idContentMap.get(id)
    sourcesContent[i] = sourceContent
  })

  map.sourcesContent = sourcesContent
}

const normalizeMap = (map) => {
  map.sourcesContent.forEach((content, i) => {
    map.sourcesContent[i] = content === '' ? null : content
  })

  return map
}

class SpeedySourceMap extends SourceMap {
  constructor() {
    super()
    this.sourceContentMap = new Map()
  }

  getNativeMap() {
    return normalizeMap(JSON.parse(super.toString()))
  }

  toMap() {
    const map = this.getNativeMap()
    bringBackSourcesContent(map, this.sourceContentMap)
    return map
  }
  toString() {
    return JSON.stringify(this.toMap())
  }
  toComment() {
    const map = this.getNativeMap()
    map.sources.forEach((source, i) => {
      let content
      if ((content = this.sourceContentMap.get(source))) {
        super.setSourceContent(i, content)
      }
    })
    return '//# sourceMappingURL=' + super.toUrl()
  }
  static mergeMaps(vlqMaps) {
    const sourceContentMap = new Map()
    const instance = SourceMap.mergeMaps(
      vlqMaps.map((map) => ensureAndStoreSourcesContent(map, sourceContentMap)),
    )

    Object.defineProperty(instance, 'sourceContentMap', {
      value: sourceContentMap,
      enumerable: false,
    })

    Object.defineProperty(instance, 'toMap', {
      value: SpeedySourceMap.prototype.toMap.bind(instance),
    })

    Object.defineProperty(instance, 'toComment', {
      value: SpeedySourceMap.prototype.toComment.bind(instance),
    })

    Object.defineProperty(instance, 'toString', {
      value: SpeedySourceMap.prototype.toString.bind(instance),
    })

    Object.defineProperty(instance, 'getNativeMap', {
      value: SpeedySourceMap.prototype.getNativeMap.bind(instance),
    })

    return instance
  }
}

module.exports = SpeedySourceMap
module.exports.default = SpeedySourceMap
module.exports.SourceMap = SpeedySourceMap
