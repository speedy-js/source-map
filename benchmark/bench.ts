import { Suite, Deferred } from 'benchmark'
import fs from "fs"
import path from "path"

import terser from "terser"
import remapping from "./fixtures/parcel"
import { SourceMap } from "../node"

const asyncTest = (fn: () => void) => ({
    defer: true,
    fn: async (deferred: Deferred) => {
        await fn();
        deferred.resolve();
    },
});

(async () => {
    const suite = new Suite('mergeMap');
    const transformedMap = fs.readFileSync(path.resolve(__dirname, './fixtures/antd/antd.js.map'), 'utf8');
    const minifiedMap = fs.readFileSync(path.resolve(__dirname, "./fixtures/antd/antd.min.js.map"), 'utf-8');

    suite
        .add(
            'antd#@speedy-js/remapping',
            asyncTest(async () => {
                await remapping([minifiedMap, transformedMap]);
            })
        )
        .add('antd#@speedy-js/source-map', () => {
            SourceMap.mergeMaps([minifiedMap, transformedMap])
        })
        .on('cycle', function (event: Event) {
            console.info(String(event.target));
        })
        .on('complete', function (this: any) {
            console.info(`${this.name} bench suite: Fastest is ${this.filter('fastest').map('name')}`);
        })
        .run();
})();

