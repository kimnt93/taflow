# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 90.53M | 0.010 | 105.24M | 0.032 | 2.92× | 3.39× |
| 10,000 | 0.061 | 164.31M | 0.058 | 172.20M | 0.078 | 1.29× | 1.35× |
| 100,000 | 0.552 | 181.02M | 0.543 | 184.19M | 0.540 | 0.98× | 1.00× |
| 1,000,000 | 6.275 | 159.37M | 5.714 | 175.01M | 5.394 | 0.86× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.141 | 1.41× |
| 1 | 5 | 0.324 | 0.499 | 1.54× |
| 1 | 10 | 0.531 | 0.930 | 1.75× |
| 10 | 1 | 0.055 | 0.095 | 1.73× |
| 10 | 5 | 0.262 | 0.451 | 1.72× |
| 10 | 10 | 0.519 | 0.961 | 1.85× |
| 100 | 1 | 0.061 | 0.102 | 1.67× |
| 100 | 5 | 0.248 | 0.468 | 1.89× |
| 100 | 10 | 0.529 | 0.983 | 1.86× |
| 1,000 | 1 | 0.059 | 0.108 | 1.82× |
| 1,000 | 5 | 0.275 | 0.500 | 1.82× |
| 1,000 | 10 | 0.631 | 1.130 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
