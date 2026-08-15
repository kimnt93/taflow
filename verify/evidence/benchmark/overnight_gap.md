# OvernightGap benchmark (`OvernightGap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.02M | 0.005 | 187.79M | 0.358 | 39.05× | 67.27× |
| 10,000 | 0.048 | 206.90M | 0.041 | 245.61M | 2.257 | 46.69× | 55.43× |
| 100,000 | 0.398 | 251.12M | 0.372 | 268.73M | 22.211 | 55.78× | 59.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.318 | 3.57× |
| 1 | 5 | 0.253 | 1.128 | 4.46× |
| 1 | 10 | 0.416 | 2.536 | 6.10× |
| 10 | 1 | 0.052 | 0.223 | 4.28× |
| 10 | 5 | 0.183 | 1.101 | 6.01× |
| 10 | 10 | 0.446 | 2.381 | 5.34× |
| 100 | 1 | 0.054 | 0.234 | 4.38× |
| 100 | 5 | 0.212 | 1.419 | 6.70× |
| 100 | 10 | 0.401 | 2.542 | 6.33× |
| 1,000 | 1 | 0.052 | 0.443 | 8.59× |
| 1,000 | 5 | 0.248 | 2.474 | 9.98× |
| 1,000 | 10 | 0.400 | 4.710 | 11.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
