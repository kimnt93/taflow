# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 35.14M | 0.026 | 38.35M | 0.038 | 1.33× | 1.45× |
| 10,000 | 0.263 | 37.98M | 0.260 | 38.49M | 0.137 | 0.52× | 0.53× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.041 ms**; native kernel **0.039 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.532 | 0.348 | 2.88M | 41.351 | 118.91× | 82.22× |
| 1,500 | 10 | 2.832 | 1.411 | 7.09M | 40.758 | 28.88× | 22.54× |
| 1,500 | 100 | 13.824 | 9.313 | 10.74M | 56.510 | 6.07× | 3.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
