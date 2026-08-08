# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 188.82M | 0.004 | 234.36M | 0.034 | 6.42× | 7.97× |
| 10,000 | 0.036 | 274.92M | 0.033 | 303.12M | 0.051 | 1.40× | 1.54× |
| 100,000 | 0.341 | 293.26M | 0.321 | 311.62M | 0.222 | 0.65× | 0.69× |
| 1,000,000 | 3.634 | 275.20M | 3.411 | 293.16M | 2.117 | 0.58× | 0.62× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.344 ms**; native kernel **0.316 ms**; TA-Lib 0.224 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.233 | 0.153 | 6.54M | 236.471 | 1547.33× | 249.74× |
| 100,000 | 10 | 1.035 | 0.533 | 18.77M | 236.899 | 444.72× | 60.30× |
| 100,000 | 1,000 | 6.357 | 4.666 | 214.32M | 231.492 | 49.61× | 7.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 198.32M | 240.20M | 1.00× | 2.96M | 2.85M | 1.00× | 271.80M |
| 2 | 354.70M | 414.44M | 1.73× | 2.97M | 3.87M | 1.36× | 291.17M |
| 4 | 496.08M | 800.89M | 3.33× | 2.79M | 3.26M | 1.14× | 288.69M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
