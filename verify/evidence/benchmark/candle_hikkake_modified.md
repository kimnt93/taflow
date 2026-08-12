# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.27M | 0.011 | 91.04M | 0.034 | 2.47× | 3.07× |
| 10,000 | 0.064 | 155.64M | 0.064 | 156.98M | 0.085 | 1.32× | 1.33× |
| 100,000 | 0.611 | 163.63M | 0.575 | 173.99M | 0.583 | 0.95× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.123 | 1.44× |
| 1 | 5 | 0.330 | 0.478 | 1.45× |
| 1 | 10 | 0.528 | 0.959 | 1.82× |
| 10 | 1 | 0.058 | 0.092 | 1.60× |
| 10 | 5 | 0.279 | 0.499 | 1.79× |
| 10 | 10 | 0.541 | 0.920 | 1.70× |
| 100 | 1 | 0.056 | 0.088 | 1.55× |
| 100 | 5 | 0.247 | 0.425 | 1.72× |
| 100 | 10 | 0.597 | 0.949 | 1.59× |
| 1,000 | 1 | 0.070 | 0.099 | 1.42× |
| 1,000 | 5 | 0.261 | 0.467 | 1.79× |
| 1,000 | 10 | 0.545 | 1.071 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
