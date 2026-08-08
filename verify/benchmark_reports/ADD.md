# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.273 | 3.66M | 0.001 | 863.36M | 0.031 | 0.11× | 26.57× |
| 10,000 | 2.741 | 3.65M | 0.004 | 2.32G | 0.032 | 0.01× | 7.52× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.390 ms**; native kernel **0.001 ms**; TA-Lib 0.030 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.328 | 0.208 | 4.80M | 29.153 | 139.97× | 128.81× |
| 1,500 | 10 | 4.526 | 0.868 | 11.52M | 29.839 | 34.36× | 32.47× |
| 1,500 | 100 | 27.211 | 2.346 | 42.63M | 31.464 | 13.41× | 11.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
