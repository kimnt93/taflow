# CandleEveningStar benchmark (`CDLEVENINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.73M | 0.004 | 280.09M | 0.038 | 5.05× | 10.58× |
| 10,000 | 0.068 | 147.59M | 0.060 | 166.93M | 0.118 | 1.74× | 1.96× |
| 100,000 | 0.887 | 112.79M | 0.891 | 112.18M | 0.860 | 0.97× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.111 | 1.57× |
| 1 | 5 | 0.290 | 0.486 | 1.68× |
| 1 | 10 | 0.403 | 0.983 | 2.44× |
| 10 | 1 | 0.043 | 0.087 | 2.00× |
| 10 | 5 | 0.183 | 0.448 | 2.45× |
| 10 | 10 | 0.401 | 1.099 | 2.74× |
| 100 | 1 | 0.124 | 0.126 | 1.02× |
| 100 | 5 | 0.239 | 0.503 | 2.11× |
| 100 | 10 | 0.463 | 1.083 | 2.34× |
| 1,000 | 1 | 0.057 | 0.113 | 1.99× |
| 1,000 | 5 | 0.225 | 0.503 | 2.23× |
| 1,000 | 10 | 0.402 | 1.043 | 2.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
