# LowestSince benchmark (`lowest since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.90M | 0.008 | 120.10M | 0.305 | 37.73× | 36.57× |
| 10,000 | 0.040 | 248.99M | 0.036 | 277.49M | 2.713 | 67.56× | 75.29× |
| 100,000 | 0.368 | 271.53M | 0.491 | 203.50M | 26.500 | 71.95× | 53.93× |
| 1,000,000 | 6.235 | 160.38M | 5.373 | 186.13M | 265.504 | 42.58× | 49.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.149 | 0.149 | 1.00× |
| 1 | 5 | 0.381 | 0.347 | 0.91× |
| 1 | 10 | 0.466 | 0.643 | 1.38× |
| 10 | 1 | 0.052 | 0.071 | 1.36× |
| 10 | 5 | 0.216 | 0.327 | 1.51× |
| 10 | 10 | 0.468 | 0.705 | 1.51× |
| 100 | 1 | 0.052 | 0.091 | 1.74× |
| 100 | 5 | 0.220 | 0.455 | 2.07× |
| 100 | 10 | 0.485 | 0.945 | 1.95× |
| 1,000 | 1 | 0.053 | 0.350 | 6.59× |
| 1,000 | 5 | 0.247 | 1.714 | 6.95× |
| 1,000 | 10 | 0.489 | 3.407 | 6.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
