# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.92M | 0.008 | 118.04M | 0.039 | 3.72× | 4.58× |
| 10,000 | 0.065 | 153.82M | 0.062 | 162.09M | 0.124 | 1.91× | 2.02× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.406 | 0.324 | 3.09M | 43.510 | 134.27× | 90.23× |
| 1,500 | 10 | 2.590 | 1.263 | 7.92M | 42.991 | 34.04× | 23.28× |
| 1,500 | 100 | 5.657 | 13.868 | 7.21M | 42.192 | 3.04× | 2.15× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.25M | 12.47M | 1.00× | 1.23M | 1.30M | 1.00× | 8.68M |
| 2 | 12.78M | 19.23M | 1.54× | 1.39M | 1.51M | 1.17× | 8.82M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
