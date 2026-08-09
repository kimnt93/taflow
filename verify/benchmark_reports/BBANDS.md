# BollingerBands benchmark (`BBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 168.91M | 0.005 | 214.31M | 0.050 | 8.36× | 10.61× |
| 10,000 | 0.042 | 239.46M | 0.036 | 277.46M | 0.092 | 2.21× | 2.56× |
| 100,000 | 0.401 | 249.36M | 0.333 | 300.60M | 0.514 | 1.28× | 1.54× |
| 1,000,000 | 15.092 | 66.26M | 7.106 | 140.73M | 9.622 | 0.64× | 1.35× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.411 ms**; native kernel **0.329 ms**; TA-Lib 0.511 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.357 | 0.265 | 3.77M | 510.276 | 1925.40× | 165.59× |
| 100,000 | 10 | 1.481 | 1.272 | 7.86M | 531.854 | 418.04× | 34.70× |
| 100,000 | 1,000 | 88.978 | 71.878 | 13.91M | 510.112 | 7.10× | 0.69× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 157.92M | 199.90M | 1.00× | 1.86M | 1.77M | 1.00× | 142.38M |
| 2 | 243.81M | 436.17M | 2.18× | 1.79M | 1.56M | 0.88× | 131.69M |
| 4 | 261.84M | 508.08M | 2.54× | 1.36M | 1.30M | 0.74× | 133.45M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
