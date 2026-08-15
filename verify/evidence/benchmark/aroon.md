# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.60M | 0.009 | 108.10M | 0.046 | 4.40× | 5.02× |
| 10,000 | 0.128 | 78.34M | 0.116 | 86.00M | 0.156 | 1.22× | 1.34× |
| 100,000 | 1.178 | 84.88M | 1.120 | 89.29M | 1.165 | 0.99× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.116 | 1.38× |
| 1 | 5 | 0.299 | 0.524 | 1.75× |
| 1 | 10 | 0.433 | 0.998 | 2.30× |
| 10 | 1 | 0.047 | 0.097 | 2.09× |
| 10 | 5 | 0.186 | 0.458 | 2.46× |
| 10 | 10 | 0.405 | 1.034 | 2.55× |
| 100 | 1 | 0.045 | 0.101 | 2.23× |
| 100 | 5 | 0.199 | 0.478 | 2.40× |
| 100 | 10 | 0.429 | 0.996 | 2.32× |
| 1,000 | 1 | 0.061 | 0.129 | 2.13× |
| 1,000 | 5 | 0.232 | 0.570 | 2.46× |
| 1,000 | 10 | 0.437 | 1.131 | 2.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
