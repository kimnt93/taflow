# MarketFacilitationIndex benchmark (`MarketFacilitationIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 24.09M | 0.034 | 29.29M | 0.185 | 4.45× | 5.41× |
| 10,000 | 0.261 | 38.32M | 0.251 | 39.77M | 0.979 | 3.75× | 3.89× |
| 100,000 | 2.762 | 36.20M | 2.521 | 39.67M | 8.986 | 3.25× | 3.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.228 | 1.84× |
| 1 | 5 | 0.430 | 1.112 | 2.58× |
| 1 | 10 | 0.629 | 1.864 | 2.96× |
| 10 | 1 | 0.071 | 0.173 | 2.45× |
| 10 | 5 | 0.315 | 0.804 | 2.55× |
| 10 | 10 | 0.629 | 1.909 | 3.04× |
| 100 | 1 | 0.080 | 0.171 | 2.14× |
| 100 | 5 | 0.297 | 0.851 | 2.86× |
| 100 | 10 | 0.612 | 1.969 | 3.22× |
| 1,000 | 1 | 0.111 | 0.259 | 2.34× |
| 1,000 | 5 | 0.307 | 1.289 | 4.20× |
| 1,000 | 10 | 0.677 | 2.562 | 3.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
