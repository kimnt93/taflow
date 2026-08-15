# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.114 | 8.78M | 0.112 | 8.94M | 0.127 | 1.11× | 1.14× |
| 10,000 | 1.090 | 9.17M | 1.106 | 9.04M | 0.791 | 0.73× | 0.72× |
| 100,000 | 11.249 | 8.89M | 11.144 | 8.97M | 7.486 | 0.67× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.119 | 1.23× |
| 1 | 5 | 0.253 | 0.511 | 2.02× |
| 1 | 10 | 0.447 | 1.076 | 2.41× |
| 10 | 1 | 0.048 | 0.096 | 2.01× |
| 10 | 5 | 0.204 | 0.479 | 2.34× |
| 10 | 10 | 0.419 | 1.067 | 2.54× |
| 100 | 1 | 0.063 | 0.112 | 1.79× |
| 100 | 5 | 0.221 | 0.527 | 2.38× |
| 100 | 10 | 0.456 | 1.078 | 2.36× |
| 1,000 | 1 | 0.164 | 0.183 | 1.12× |
| 1,000 | 5 | 0.370 | 0.920 | 2.49× |
| 1,000 | 10 | 0.557 | 1.758 | 3.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
