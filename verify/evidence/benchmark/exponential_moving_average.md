# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 278.40M | 0.003 | 369.80M | 0.032 | 8.92× | 11.84× |
| 10,000 | 0.021 | 480.18M | 0.018 | 544.45M | 0.059 | 2.82× | 3.20× |
| 100,000 | 0.201 | 496.92M | 0.176 | 567.94M | 0.290 | 1.44× | 1.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.055 | 0.119 | 2.18× |
| 1 | 5 | 0.220 | 0.530 | 2.41× |
| 1 | 10 | 0.409 | 0.905 | 2.21× |
| 10 | 1 | 0.044 | 0.085 | 1.93× |
| 10 | 5 | 0.182 | 0.433 | 2.38× |
| 10 | 10 | 0.392 | 0.897 | 2.29× |
| 100 | 1 | 0.039 | 0.088 | 2.25× |
| 100 | 5 | 0.174 | 0.433 | 2.48× |
| 100 | 10 | 0.366 | 0.906 | 2.47× |
| 1,000 | 1 | 0.043 | 0.090 | 2.07× |
| 1,000 | 5 | 0.195 | 0.464 | 2.38× |
| 1,000 | 10 | 0.402 | 0.924 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
