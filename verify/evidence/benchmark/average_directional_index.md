# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.50M | 0.011 | 91.26M | 0.042 | 3.95× | 3.81× |
| 10,000 | 0.079 | 127.11M | 0.094 | 105.88M | 0.125 | 1.59× | 1.32× |
| 100,000 | 0.809 | 123.55M | 0.906 | 110.34M | 0.916 | 1.13× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.117 | 1.26× |
| 1 | 5 | 0.281 | 0.499 | 1.77× |
| 1 | 10 | 0.391 | 0.935 | 2.39× |
| 10 | 1 | 0.041 | 0.090 | 2.21× |
| 10 | 5 | 0.196 | 0.470 | 2.40× |
| 10 | 10 | 0.412 | 0.967 | 2.35× |
| 100 | 1 | 0.044 | 0.091 | 2.07× |
| 100 | 5 | 0.206 | 0.445 | 2.16× |
| 100 | 10 | 0.415 | 1.017 | 2.45× |
| 1,000 | 1 | 0.051 | 0.107 | 2.10× |
| 1,000 | 5 | 0.204 | 0.494 | 2.42× |
| 1,000 | 10 | 0.401 | 1.065 | 2.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
