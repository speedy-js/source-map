import SourceMap, { VLQMap, SourceMapStringifyOptions } from '@speedy-js/parcel-source-map';

export interface RemappingOptions {}

export interface RemappingResult {
    toBuffer: () => Buffer;
    toMap: (options?: ToMapOptions) => Promise<VLQMap>;
    toString: (options?: ToStringOptions) => Promise<string>;
}

export interface ToStringOptions extends Omit<SourceMapStringifyOptions, 'format'> {}
export interface ToMapOptions extends Omit<SourceMapStringifyOptions, 'format'> {}

// This fixes an issue where a null is included in the source map as which cannot be recognized by Parcel SourceMap.
function normalizeSourceMap(map: VLQMap): VLQMap {
    return Object.assign(map, {
        sources: (map.sources || []).map((s, index) => (s === null ? String(index) : s)),
    });
}
function ensureSourceMap(map: string | VLQMap): VLQMap {
    return normalizeSourceMap(typeof map === 'string' ? JSON.parse(map) : map);
}

type Hash = string;
const mapCache = new Map<Hash, SourceMap>();

export default function remapping(sourceMapList: string[] | any[], options: RemappingOptions = {}): RemappingResult {
    if (sourceMapList.length === 0) throw new Error('No source map found');

    let innerMapList = sourceMapList.map((map) => {
        return {
            map: ensureSourceMap(map) as VLQMap,
        };
    });
    let mergedMap: SourceMap;

    if (innerMapList.length === 1) {
        mergedMap = new SourceMap();
        mergedMap.addVLQMap(innerMapList[0].map);
    } else {
        const last = innerMapList[innerMapList.length - 1];
        mergedMap = new SourceMap();
        mergedMap.addVLQMap(last.map);

        mergedMap = innerMapList
            .slice(0, -1)
            .map((item) => item.map)
            .reduceRight((originalSourceMap: SourceMap, curr: VLQMap) => {
                const map = new SourceMap();
                map.addVLQMap(curr);

                if (originalSourceMap) {
                    map.extends(originalSourceMap.toBuffer());
                }

                return map;
            }, mergedMap);
    }

    const toMap = async (options = {}) => {
        const mapObject = (await mergedMap.stringify({
            inlineSources: true,
            ...options,
            format: 'object',
        })) as VLQMap;

        // TODO: Since Parcel Sourcemap does not help us with middle-output shakings, we need to manually remove the nulls.
        // We have to shake middle-output sources and sourcesContent since we don't consume them,
        // which significantly reduces the size of the source map and also makes it faster to stringify.

        return mapObject;
    };

    const toString = async (options = {}) => {
        return JSON.stringify(await toMap(options));
    };

    return {
        toBuffer: () => mergedMap.toBuffer(),
        toMap,
        toString,
    };
}