# TimeSeriesRank benchmark (`rolling percentile rank` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.86M | 0.018 | 55.74M | 0.137 | 7.50× | 7.62× |
| 10,000 | 0.163 | 61.19M | 0.174 | 57.46M | 0.719 | 4.40× | 4.13× |
| 100,000 | 1.581 | 63.27M | 1.648 | 60.69M | 6.666 | 4.22× | 4.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.135 | 1.79× |
| 1 | 5 | 0.238 | 0.539 | 2.27× |
| 1 | 10 | 0.397 | 1.061 | 2.67× |
| 10 | 1 | 0.044 | 0.113 | 2.58× |
| 10 | 5 | 0.184 | 0.516 | 2.80× |
| 10 | 10 | 0.411 | 1.060 | 2.58× |
| 100 | 1 | 0.048 | 0.161 | 3.33× |
| 100 | 5 | 0.196 | 0.761 | 3.89× |
| 100 | 10 | 0.469 | 1.667 | 3.56× |
| 1,000 | 1 | 0.063 | 0.214 | 3.37× |
| 1,000 | 5 | 0.224 | 0.969 | 4.32× |
| 1,000 | 10 | 0.431 | 1.992 | 4.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
