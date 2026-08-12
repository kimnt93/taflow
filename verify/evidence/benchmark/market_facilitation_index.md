# MarketFacilitationIndex benchmark (`MarketFacilitationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.65M | 0.007 | 140.02M | 0.199 | 20.65× | 27.90× |
| 10,000 | 0.033 | 305.32M | 0.028 | 351.61M | 1.092 | 33.35× | 38.40× |
| 100,000 | 0.224 | 447.14M | 0.188 | 531.89M | 9.874 | 44.15× | 52.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.236 | 2.51× |
| 1 | 5 | 0.334 | 1.185 | 3.55× |
| 1 | 10 | 0.556 | 2.054 | 3.69× |
| 10 | 1 | 0.060 | 0.166 | 2.78× |
| 10 | 5 | 0.253 | 0.827 | 3.27× |
| 10 | 10 | 0.564 | 2.098 | 3.72× |
| 100 | 1 | 0.059 | 0.181 | 3.07× |
| 100 | 5 | 0.242 | 0.889 | 3.68× |
| 100 | 10 | 0.548 | 2.181 | 3.98× |
| 1,000 | 1 | 0.057 | 0.265 | 4.68× |
| 1,000 | 5 | 0.267 | 1.357 | 5.07× |
| 1,000 | 10 | 0.632 | 2.737 | 4.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
