# ForceIndex benchmark (`ForceIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 153.96M | 0.005 | 213.37M | 0.202 | 31.10× | 43.10× |
| 10,000 | 0.040 | 251.22M | 0.038 | 265.74M | 0.774 | 19.44× | 20.56× |
| 100,000 | 0.354 | 282.27M | 0.346 | 289.16M | 6.345 | 17.91× | 18.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.238 | 3.45× |
| 1 | 5 | 0.277 | 1.110 | 4.01× |
| 1 | 10 | 0.427 | 2.826 | 6.62× |
| 10 | 1 | 0.049 | 0.215 | 4.35× |
| 10 | 5 | 0.189 | 1.354 | 7.18× |
| 10 | 10 | 0.436 | 2.716 | 6.23× |
| 100 | 1 | 0.046 | 0.225 | 4.85× |
| 100 | 5 | 0.196 | 1.383 | 7.05× |
| 100 | 10 | 0.415 | 2.385 | 5.75× |
| 1,000 | 1 | 0.048 | 0.283 | 5.96× |
| 1,000 | 5 | 0.201 | 1.636 | 8.15× |
| 1,000 | 10 | 0.452 | 2.985 | 6.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
