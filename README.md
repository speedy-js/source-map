<h1>Speedy SourceMap</h1>


<p>
<a href="https://github.com/speedy-js/source-map/actions/workflows/CI.yaml"><img src="https://github.com/speedy-js/source-map/actions/workflows/CI.yaml/badge.svg" alt="CI"></a>
<a href="https://crates.io/crates/speedy_sourcemap"><img src="https://img.shields.io/crates/v/speedy_sourcemap.svg?label=crates" alt="crates"></a>
<a href="https://www.npmjs.com/package/@speedy-js/source-map"><img src="https://img.shields.io/npm/v/@speedy-js/source-map?color=666&amp;label=NPM" alt="NPM version"></a>
</p>

SourceMap kit for SpeedyStack

> SourceMap is under heavy development, API might change in the future. DO NOT use it in the production.

## Benchmark

**mergeMaps(remapping)**
```
lottie#@speedy-js/source-map - parallel x 128 ops/sec ±0.79% (82 runs sampled)
lottie#@speedy-js/remapping x 73.20 ops/sec ±0.83% (85 runs sampled)
lottie#@ampremapping x 40.42 ops/sec ±6.19% (56 runs sampled)
mergeMap#lottie bench suite: Fastest is lottie#@speedy-js/source-map - parallel


antd#@speedy-js/source-map  - parallel x 12.10 ops/sec ±1.06% (34 runs sampled)
antd#@speedy-js/remapping x 9.95 ops/sec ±0.53% (51 runs sampled)
antd@ampremapping x 4.79 ops/sec ±4.00% (17 runs sampled)
mergeMap#antd bench suite: Fastest is antd#@speedy-js/source-map  - parallel
```

## Documentations

Rust 📦 - [docs.rs](https://docs.rs/speedy_sourcemap)


## License

MIT © h-a-n-a
