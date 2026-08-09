# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.86M | 0.004 | 241.85M | 0.032 | 5.27× | 7.69× |
| 10,000 | 0.073 | 136.37M | 0.070 | 143.22M | 0.117 | 1.59× | 1.67× |
| 100,000 | 0.850 | 117.70M | 0.847 | 118.01M | 0.955 | 1.12× | 1.13× |
| 1,000,000 | 8.921 | 112.09M | 8.907 | 112.27M | 9.392 | 1.05× | 1.05× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.847 ms**; native kernel **0.858 ms**; TA-Lib 0.957 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.326 | 0.266 | 3.76M | 945.806 | 3556.68× | 101.32× |
| 100,000 | 10 | 2.572 | 1.366 | 7.32M | 949.861 | 695.43× | 20.59× |
| 100,000 | 1,000 | 28.655 | 24.571 | 40.70M | 967.131 | 39.36× | 1.28× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 98.96M | 99.47M | 1.00× | 2.05M | 2.81M | 1.00× | 88.67M |
| 2 | 186.86M | 196.27M | 1.97× | 2.39M | 2.64M | 0.94× | 90.30M |
| 4 | 314.40M | 372.05M | 3.74× | 2.21M | 2.56M | 0.91× | 89.48M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
