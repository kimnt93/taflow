# OvernightGap benchmark (`OvernightGap` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.57M | 0.005 | 182.88M | 0.370 | 38.34× | 67.71× |
| 10,000 | 0.048 | 206.79M | 0.041 | 241.76M | 2.368 | 48.97× | 57.25× |
| 100,000 | 0.406 | 246.60M | 0.376 | 266.03M | 22.482 | 55.44× | 59.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.281 | 3.30× |
| 1 | 5 | 0.364 | 1.175 | 3.23× |
| 1 | 10 | 0.487 | 2.594 | 5.33× |
| 10 | 1 | 0.044 | 0.221 | 4.98× |
| 10 | 5 | 0.197 | 1.111 | 5.64× |
| 10 | 10 | 0.437 | 2.366 | 5.41× |
| 100 | 1 | 0.054 | 0.236 | 4.38× |
| 100 | 5 | 0.188 | 1.435 | 7.64× |
| 100 | 10 | 0.458 | 2.641 | 5.76× |
| 1,000 | 1 | 0.063 | 0.479 | 7.56× |
| 1,000 | 5 | 0.220 | 2.453 | 11.17× |
| 1,000 | 10 | 0.444 | 4.667 | 10.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
