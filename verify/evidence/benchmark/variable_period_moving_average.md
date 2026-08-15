# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.093 | 10.70M | 0.110 | 9.11M | 0.118 | 1.26× | 1.07× |
| 10,000 | 0.816 | 12.25M | 0.821 | 12.17M | 0.802 | 0.98× | 0.98× |
| 100,000 | 8.909 | 11.22M | 8.951 | 11.17M | 7.560 | 0.85× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.124 | 1.00× |
| 1 | 5 | 0.324 | 0.523 | 1.61× |
| 1 | 10 | 0.429 | 1.007 | 2.35× |
| 10 | 1 | 0.047 | 0.096 | 2.03× |
| 10 | 5 | 0.206 | 0.499 | 2.43× |
| 10 | 10 | 0.460 | 1.046 | 2.27× |
| 100 | 1 | 0.058 | 0.102 | 1.77× |
| 100 | 5 | 0.214 | 0.512 | 2.39× |
| 100 | 10 | 0.439 | 1.150 | 2.62× |
| 1,000 | 1 | 0.140 | 0.184 | 1.31× |
| 1,000 | 5 | 0.279 | 0.877 | 3.14× |
| 1,000 | 10 | 0.511 | 1.839 | 3.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
