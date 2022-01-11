import { Suite, Deferred } from 'benchmark'
import chalk from 'chalk'
import fs from 'fs'
import path from 'path'

import convertSourceMap from 'convert-source-map'
import ampremapping from '@ampproject/remapping'
import mergeSourceMap from 'merge-source-map'
import remapping from './fixtures/parcel'
import { SourceMap } from '../node'

const asyncTest = (fn: () => void) => ({
  defer: true,
  fn: async (deferred: Deferred) => {
    await fn()
    deferred.resolve()
  },
})

const bench1 = async () => {
  const suite = new Suite('mergeMap#lottie')
  const transformedMap = fs.readFileSync(
    path.resolve(__dirname, './fixtures/lottie/lottie.es.js.map'),
    'utf-8',
  )
  const minifiedMap = fs.readFileSync(
    path.resolve(__dirname, './fixtures/lottie/lottie.es.min.js.map'),
    'utf-8',
  )

  return new Promise<void>((res) => {
    suite
      .add('lottie#@speedy-js/source-map - parallel', () => {
        SourceMap.mergeMaps([minifiedMap, transformedMap])
      })
      .add(
        'lottie#@speedy-js/remapping',
        asyncTest(async () => {
          await remapping([minifiedMap, transformedMap])
        }),
      )
      .add('lottie#@ampremapping', () => {
        ampremapping([minifiedMap, transformedMap], () => null)
      })
      .add('lottie#merge-source-map', () => {
        mergeSourceMap(transformedMap, minifiedMap)
      })
      .on('cycle', function (event: Event) {
        console.info(String(event.target))
      })
      .on('complete', function (this: any) {
        console.info(
          `${this.name} bench suite: Fastest is ${chalk.green(
            this.filter('fastest').map('name'),
          )}\n\n`,
        )
        res()
      })
      .run()
  })
}

const bench2 = async () => {
  const suite = new Suite('mergeMap#antd')
  const transformedMap = fs.readFileSync(
    path.resolve(__dirname, './fixtures/antd/antd.js.map'),
    'utf-8',
  )
  const minifiedMap = fs.readFileSync(
    path.resolve(__dirname, './fixtures/antd/antd.min.js.map'),
    'utf-8',
  )

  return new Promise<void>((res) => {
    suite
      .add('antd#@speedy-js/source-map  - parallel', () => {
        SourceMap.mergeMaps([minifiedMap, transformedMap])
      })
      .add(
        'antd#@speedy-js/remapping',
        asyncTest(async () => {
          await remapping([minifiedMap, transformedMap])
        }),
      )
      .add('antd@ampremapping', () => {
        ampremapping([minifiedMap, transformedMap], () => null)
      })
      .add('antd#merge-source-map', () => {
        mergeSourceMap(transformedMap, minifiedMap)
      })
      .on('cycle', function (event: Event) {
        console.info(String(event.target))
      })
      .on('complete', function (this: any) {
        console.info(
          `${this.name} bench suite: Fastest is ${chalk.green(
            this.filter('fastest').map('name'),
          )}\n\n`,
        )
        res()
      })
      .run()
  })
}

const bench3 = async () => {
  const suite = new Suite('mergeMap.toString#lottie')
  const transformedMap = fs.readFileSync(
    path.resolve(__dirname, './fixtures/lottie/lottie.es.js.map'),
    'utf-8',
  )
  const minifiedMap = fs.readFileSync(
    path.resolve(__dirname, './fixtures/lottie/lottie.es.min.js.map'),
    'utf-8',
  )

  return new Promise<void>((res) => {
    suite
      .add('lottie#@speedy-js/source-map - parallel', () => {
        SourceMap.mergeMaps([minifiedMap, transformedMap]).toString()
      })
      .add(
        'lottie#@speedy-js/remapping',
        asyncTest(async () => {
          await remapping([minifiedMap, transformedMap]).toString()
        }),
      )
      .add('lottie#@ampremapping', () => {
        ampremapping([minifiedMap, transformedMap], () => null).toString()
      })
      .add('lottie#merge-source-map', () => {
        mergeSourceMap(transformedMap, minifiedMap).toString()
      })
      .on('cycle', function (event: Event) {
        console.info(String(event.target))
      })
      .on('complete', function (this: any) {
        console.info(
          `${this.name} bench suite: Fastest is ${chalk.green(
            this.filter('fastest').map('name'),
          )}\n\n`,
        )
        res()
      })
      .run()
  })
}

const bench4 = async () => {
  const suite = new Suite('mergeMap.toMap#lottie')
  const transformedMap = fs.readFileSync(
    path.resolve(__dirname, './fixtures/lottie/lottie.es.js.map'),
    'utf-8',
  )
  const minifiedMap = fs.readFileSync(
    path.resolve(__dirname, './fixtures/lottie/lottie.es.min.js.map'),
    'utf-8',
  )

  return new Promise<void>((res) => {
    suite
      .add('lottie#@speedy-js/source-map - parallel', () => {
        SourceMap.mergeMaps([minifiedMap, transformedMap]).toMap()
      })
      .add(
        'lottie#@speedy-js/remapping',
        asyncTest(async () => {
          await remapping([minifiedMap, transformedMap]).toMap()
        }),
      )
      .add('lottie#@ampremapping', () => {
        ampremapping([minifiedMap, transformedMap], () => null)
      })
      .add('lottie#merge-source-map', () => {
        mergeSourceMap(transformedMap, minifiedMap)
      })
      .on('cycle', function (event: Event) {
        console.info(String(event.target))
      })
      .on('complete', function (this: any) {
        console.info(
          `${this.name} bench suite: Fastest is ${chalk.green(
            this.filter('fastest').map('name'),
          )}\n\n`,
        )
        res()
      })
      .run()
  })
}

const bench5 = async () => {
  const suite = new Suite('mergeMap.toComment#lottie')
  const transformedMap = JSON.parse(
    fs.readFileSync(
      path.resolve(__dirname, './fixtures/lottie/lottie.es.js.map'),
      'utf-8',
    ),
  )
  const minifiedMap = JSON.parse(
    fs.readFileSync(
      path.resolve(__dirname, './fixtures/lottie/lottie.es.min.js.map'),
      'utf-8',
    ),
  )

  return new Promise<void>((res) => {
    suite
      .add('lottie#@speedy-js/source-map - parallel', () => {
        SourceMap.mergeMaps([minifiedMap, transformedMap]).toComment()
      })
      .add(
        'lottie#@speedy-js/remapping',
        asyncTest(async () => {
          const map = await remapping([minifiedMap, transformedMap]).toMap()
          convertSourceMap.fromObject(map).toComment()
        }),
      )
      .add('lottie#@ampremapping', () => {
        const map = ampremapping([minifiedMap, transformedMap], () => null)
        convertSourceMap.fromObject(map).toComment()
      })
      .add('lottie#merge-source-map', () => {
        const map = mergeSourceMap(transformedMap, minifiedMap)
        convertSourceMap.fromObject(map).toComment()
      })
      .on('cycle', function (event: Event) {
        console.info(String(event.target))
      })
      .on('complete', function (this: any) {
        console.info(
          `${this.name} bench suite: Fastest is ${chalk.green(
            this.filter('fastest').map('name'),
          )}\n\n`,
        )
        res()
      })
      .run()
  })
}

;(async () => {
  const chainPromise = (promises: (() => Promise<any>)[]) => {
    return promises.reduce(async (prev, now) => {
      await prev
      return now()
    }, Promise.resolve())
  }

  chainPromise([bench1, bench2, bench3, bench4, bench5])
})()
