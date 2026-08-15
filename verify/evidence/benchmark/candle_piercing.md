# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 157.37M | 0.003 | 349.37M | 0.034 | 5.34× | 11.87× |
| 10,000 | 0.072 | 138.97M | 0.068 | 146.85M | 0.117 | 1.62× | 1.71× |
| 100,000 | 0.819 | 122.07M | 0.776 | 128.88M | 0.973 | 1.19× | 1.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.127 | 1.72× |
| 1 | 5 | 0.299 | 0.534 | 1.79× |
| 1 | 10 | 0.389 | 0.930 | 2.39× |
| 10 | 1 | 0.046 | 0.092 | 2.01× |
| 10 | 5 | 0.195 | 0.449 | 2.30× |
| 10 | 10 | 0.437 | 0.929 | 2.12× |
| 100 | 1 | 0.052 | 0.096 | 1.85× |
| 100 | 5 | 0.191 | 0.452 | 2.37× |
| 100 | 10 | 0.389 | 1.002 | 2.58× |
| 1,000 | 1 | 0.051 | 0.104 | 2.04× |
| 1,000 | 5 | 0.219 | 0.508 | 2.32× |
| 1,000 | 10 | 0.432 | 1.054 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
