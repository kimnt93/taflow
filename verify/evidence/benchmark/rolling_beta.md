# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.46M | 0.009 | 116.96M | 0.041 | 4.07× | 4.84× |
| 10,000 | 0.057 | 176.04M | 0.053 | 189.60M | 0.092 | 1.61× | 1.74× |
| 100,000 | 0.532 | 188.11M | 0.505 | 197.97M | 0.608 | 1.14× | 1.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.123 | 1.80× |
| 1 | 5 | 0.348 | 0.453 | 1.30× |
| 1 | 10 | 0.487 | 1.063 | 2.18× |
| 10 | 1 | 0.057 | 0.094 | 1.63× |
| 10 | 5 | 0.228 | 0.449 | 1.97× |
| 10 | 10 | 0.462 | 0.936 | 2.03× |
| 100 | 1 | 0.057 | 0.100 | 1.76× |
| 100 | 5 | 0.260 | 0.496 | 1.91× |
| 100 | 10 | 0.498 | 0.943 | 1.89× |
| 1,000 | 1 | 0.058 | 0.095 | 1.65× |
| 1,000 | 5 | 0.241 | 0.501 | 2.08× |
| 1,000 | 10 | 0.569 | 1.058 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
