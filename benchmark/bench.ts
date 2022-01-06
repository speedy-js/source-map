import { Suite, Deferred } from 'benchmark'
import chalk from "chalk"
import fs from "fs"
import path from "path"

import remapping from "./fixtures/parcel"
import ampremapping from "@ampproject/remapping"
import { SourceMap } from "../node"

const asyncTest = (fn: () => void) => ({
    defer: true,
    fn: async (deferred: Deferred) => {
        await fn();
        deferred.resolve();
    },
});

const bench1 = (async () => {
    const suite = new Suite('mergeMap#lottie');
    const transformedMap = fs.readFileSync(path.resolve(__dirname, './fixtures/lottie/lottie.es.js.map'), 'utf8');
    const minifiedMap = fs.readFileSync(path.resolve(__dirname, "./fixtures/lottie/lottie.es.min.js.map"), 'utf-8');

    suite
        .add('lottie#@speedy-js/source-map - parallel', () => {
            SourceMap.mergeMaps([minifiedMap, transformedMap])
        })
        .add(
            'lottie#@speedy-js/remapping',
            asyncTest(async () => {
                await remapping([minifiedMap, transformedMap]);
            })
        )
        .add('lottie#@ampremapping', () => {
            ampremapping([minifiedMap, transformedMap], () => null)
        })
        .on('cycle', function (event: Event) {
            console.info(String(event.target));
        })
        .on('complete', function (this: any) {
            console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}\n\n`);
        })
        .run();
});

const bench2 = (async () => {
    const suite = new Suite('mergeMap#antd');
    const transformedMap = fs.readFileSync(path.resolve(__dirname, './fixtures/antd/antd.js.map'), 'utf8');
    const minifiedMap = fs.readFileSync(path.resolve(__dirname, "./fixtures/antd/antd.min.js.map"), 'utf-8');

    suite
        .add('antd#@speedy-js/source-map  - parallel', () => {
            SourceMap.mergeMaps([minifiedMap, transformedMap])
        })
        .add(
            'antd#@speedy-js/remapping',
            asyncTest(async () => {
                await remapping([minifiedMap, transformedMap]);
            })
        )
        .add('antd@ampremapping', () => {
            ampremapping([minifiedMap, transformedMap], () => null)
        })
        .on('cycle', function (event: Event) {
            console.info(String(event.target));
        })
        .on('complete', function (this: any) {
            console.info(`${this.name} bench suite: Fastest is ${chalk.green(this.filter('fastest').map('name'))}\n\n`);
        })
        .run();
});

(async () => {
    const chainPromise = (promises: (() => Promise<any>)[]) => {
        return promises.reduce(async (prev, now) => {
            await prev;
            return now()
        }, Promise.resolve())
    }

    chainPromise([bench1, bench2])
})()