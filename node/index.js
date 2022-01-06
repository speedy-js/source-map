const { SourceMap } = require("./binding")

const ensureMap = (map) => {
    return map && typeof map === "object" ? JSON.stringify(map) : map
}

class SpeedySourceMap extends SourceMap {
    toMap() {
        return JSON.parse(super.toString())
    }
    static mergeMaps(vlqMaps) {
        const instance = SourceMap.mergeMaps(vlqMaps.map(map => ensureMap(map)))

        return Object.defineProperty(instance, "toMap", {
            value: SpeedySourceMap.prototype.toMap.bind(instance)
        })
    }
}

module.exports = SpeedySourceMap;
module.exports.default = SpeedySourceMap;
module.exports.SourceMap = SpeedySourceMap;