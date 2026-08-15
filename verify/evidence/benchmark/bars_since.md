# BarsSince benchmark (`bars since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 252.29M | 0.003 | 325.09M | 0.114 | 28.65× | 36.91× |
| 10,000 | 0.025 | 404.16M | 0.023 | 443.38M | 1.054 | 42.61× | 46.74× |
| 100,000 | 0.240 | 416.24M | 0.219 | 457.02M | 11.358 | 47.28× | 51.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.091 | 1.31× |
| 1 | 5 | 0.211 | 0.303 | 1.44× |
| 1 | 10 | 0.381 | 0.597 | 1.57× |
| 10 | 1 | 0.039 | 0.060 | 1.53× |
| 10 | 5 | 0.181 | 0.305 | 1.68× |
| 10 | 10 | 0.400 | 0.656 | 1.64× |
| 100 | 1 | 0.045 | 0.072 | 1.61× |
| 100 | 5 | 0.205 | 0.333 | 1.63× |
| 100 | 10 | 0.379 | 0.698 | 1.84× |
| 1,000 | 1 | 0.046 | 0.169 | 3.72× |
| 1,000 | 5 | 0.188 | 0.837 | 4.45× |
| 1,000 | 10 | 0.412 | 1.694 | 4.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
