# Amihud benchmark (`AmihudIlliquidity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.80M | 0.057 | 17.63M | 0.533 | 8.95× | 9.39× |
| 10,000 | 0.456 | 21.91M | 0.458 | 21.82M | 3.628 | 7.95× | 7.91× |
| 100,000 | 4.406 | 22.70M | 4.487 | 22.29M | 35.035 | 7.95× | 7.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.202 | 0.314 | 1.55× |
| 1 | 5 | 0.456 | 1.103 | 2.42× |
| 1 | 10 | 0.639 | 2.336 | 3.66× |
| 10 | 1 | 0.071 | 0.210 | 2.97× |
| 10 | 5 | 0.317 | 1.068 | 3.37× |
| 10 | 10 | 0.625 | 2.390 | 3.82× |
| 100 | 1 | 0.078 | 0.244 | 3.14× |
| 100 | 5 | 0.302 | 1.222 | 4.05× |
| 100 | 10 | 0.654 | 2.749 | 4.21× |
| 1,000 | 1 | 0.123 | 0.599 | 4.87× |
| 1,000 | 5 | 0.310 | 2.937 | 9.48× |
| 1,000 | 10 | 0.657 | 6.168 | 9.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
