# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.202 | 4.96M | 0.020 | 49.74M | 0.048 | 0.24× | 2.41× |
| 10,000 | 2.152 | 4.65M | 0.186 | 53.65M | 0.117 | 0.05× | 0.63× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.317 ms**; native kernel **0.029 ms**; TA-Lib 0.054 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.396 | 0.291 | 3.43M | 54.727 | 187.88× | 136.64× |
| 1,500 | 10 | 4.407 | 1.514 | 6.60M | 53.454 | 35.30× | 26.38× |
| 1,500 | 100 | 16.397 | 4.535 | 22.05M | 53.302 | 11.75× | 8.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
