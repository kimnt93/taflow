# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.43M | 0.017 | 58.81M | 0.038 | 1.85× | 2.25× |
| 10,000 | 0.140 | 71.47M | 0.136 | 73.64M | 0.141 | 1.01× | 1.04× |
| 100,000 | 1.407 | 71.08M | 1.422 | 70.30M | 1.168 | 0.83× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.173 | 1.88× |
| 1 | 5 | 0.421 | 0.470 | 1.11× |
| 1 | 10 | 0.572 | 0.919 | 1.61× |
| 10 | 1 | 0.059 | 0.084 | 1.42× |
| 10 | 5 | 0.259 | 0.428 | 1.65× |
| 10 | 10 | 0.557 | 0.957 | 1.72× |
| 100 | 1 | 0.061 | 0.092 | 1.50× |
| 100 | 5 | 0.253 | 0.421 | 1.67× |
| 100 | 10 | 0.556 | 0.917 | 1.65× |
| 1,000 | 1 | 0.071 | 0.104 | 1.47× |
| 1,000 | 5 | 0.279 | 0.504 | 1.80× |
| 1,000 | 10 | 0.571 | 0.998 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
