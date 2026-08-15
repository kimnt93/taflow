# HullMovingAverage benchmark (`HMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 39.76M | 0.023 | 42.97M | 0.160 | 6.34× | 6.86× |
| 10,000 | 0.221 | 45.35M | 0.212 | 47.23M | 0.525 | 2.38× | 2.48× |
| 100,000 | 1.991 | 50.24M | 1.941 | 51.53M | 4.255 | 2.14× | 2.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.143 | 0.231 | 1.61× |
| 1 | 5 | 0.275 | 0.985 | 3.58× |
| 1 | 10 | 0.413 | 2.092 | 5.07× |
| 10 | 1 | 0.049 | 0.186 | 3.76× |
| 10 | 5 | 0.191 | 0.950 | 4.96× |
| 10 | 10 | 0.385 | 2.052 | 5.33× |
| 100 | 1 | 0.048 | 0.192 | 4.04× |
| 100 | 5 | 0.226 | 0.958 | 4.23× |
| 100 | 10 | 0.436 | 2.092 | 4.79× |
| 1,000 | 1 | 0.067 | 0.230 | 3.42× |
| 1,000 | 5 | 0.195 | 1.140 | 5.84× |
| 1,000 | 10 | 0.450 | 2.481 | 5.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
