# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.22M | 0.009 | 115.71M | 0.041 | 3.42× | 4.75× |
| 10,000 | 0.138 | 72.37M | 0.130 | 77.06M | 0.173 | 1.25× | 1.33× |
| 100,000 | 1.401 | 71.37M | 1.394 | 71.73M | 1.411 | 1.01× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.118 | 1.45× |
| 1 | 5 | 0.301 | 0.453 | 1.51× |
| 1 | 10 | 0.411 | 0.977 | 2.38× |
| 10 | 1 | 0.054 | 0.090 | 1.65× |
| 10 | 5 | 0.198 | 0.434 | 2.20× |
| 10 | 10 | 0.414 | 0.939 | 2.27× |
| 100 | 1 | 0.049 | 0.109 | 2.25× |
| 100 | 5 | 0.214 | 0.461 | 2.15× |
| 100 | 10 | 0.382 | 0.951 | 2.49× |
| 1,000 | 1 | 0.071 | 0.111 | 1.55× |
| 1,000 | 5 | 0.238 | 0.553 | 2.33× |
| 1,000 | 10 | 0.419 | 1.031 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
