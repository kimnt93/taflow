# RollingSharpe benchmark (`SharpeRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.61M | 0.030 | 32.81M | 0.180 | 5.70× | 5.92× |
| 10,000 | 0.292 | 34.23M | 0.289 | 34.66M | 0.537 | 1.84× | 1.86× |
| 100,000 | 2.966 | 33.72M | 2.891 | 34.59M | 3.988 | 1.34× | 1.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.175 | 0.242 | 1.38× |
| 1 | 5 | 0.294 | 1.310 | 4.45× |
| 1 | 10 | 0.518 | 2.424 | 4.68× |
| 10 | 1 | 0.044 | 0.216 | 4.93× |
| 10 | 5 | 0.186 | 1.451 | 7.79× |
| 10 | 10 | 0.401 | 2.335 | 5.82× |
| 100 | 1 | 0.049 | 0.221 | 4.56× |
| 100 | 5 | 0.219 | 1.360 | 6.21× |
| 100 | 10 | 0.427 | 2.395 | 5.60× |
| 1,000 | 1 | 0.079 | 0.261 | 3.31× |
| 1,000 | 5 | 0.209 | 1.460 | 6.98× |
| 1,000 | 10 | 0.427 | 2.815 | 6.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
