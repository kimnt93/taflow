# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.089 | 11.22M | 0.095 | 10.52M | 0.037 | 0.42× | 0.39× |
| 10,000 | 0.727 | 13.76M | 0.720 | 13.88M | 0.126 | 0.17× | 0.17× |
| 100,000 | 7.085 | 14.11M | 7.117 | 14.05M | 0.970 | 0.14× | 0.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.139 | 1.04× |
| 1 | 5 | 0.456 | 0.484 | 1.06× |
| 1 | 10 | 0.654 | 0.900 | 1.38× |
| 10 | 1 | 0.065 | 0.089 | 1.37× |
| 10 | 5 | 0.314 | 0.439 | 1.40× |
| 10 | 10 | 0.631 | 0.901 | 1.43× |
| 100 | 1 | 0.072 | 0.086 | 1.19× |
| 100 | 5 | 0.311 | 0.435 | 1.40× |
| 100 | 10 | 0.622 | 0.912 | 1.47× |
| 1,000 | 1 | 0.141 | 0.106 | 0.75× |
| 1,000 | 5 | 0.334 | 0.482 | 1.44× |
| 1,000 | 10 | 0.701 | 1.043 | 1.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
