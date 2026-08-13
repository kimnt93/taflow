# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.34M | 0.059 | 17.04M | 0.040 | 0.57× | 0.68× |
| 10,000 | 0.517 | 19.33M | 0.525 | 19.04M | 0.085 | 0.16× | 0.16× |
| 100,000 | 4.979 | 20.08M | 4.884 | 20.48M | 0.554 | 0.11× | 0.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.124 | 1.37× |
| 1 | 5 | 0.458 | 0.541 | 1.18× |
| 1 | 10 | 0.636 | 1.000 | 1.57× |
| 10 | 1 | 0.074 | 0.097 | 1.32× |
| 10 | 5 | 0.293 | 0.469 | 1.60× |
| 10 | 10 | 0.613 | 0.975 | 1.59× |
| 100 | 1 | 0.077 | 0.097 | 1.26× |
| 100 | 5 | 0.312 | 0.471 | 1.51× |
| 100 | 10 | 0.627 | 0.989 | 1.58× |
| 1,000 | 1 | 0.123 | 0.103 | 0.84× |
| 1,000 | 5 | 0.345 | 0.527 | 1.53× |
| 1,000 | 10 | 0.681 | 1.031 | 1.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
