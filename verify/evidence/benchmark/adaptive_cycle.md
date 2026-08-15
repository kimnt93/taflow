# AdaptiveCycle benchmark (`AdaptiveCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.062 | 16.19M | 0.060 | 16.73M | 0.194 | 3.15× | 3.25× |
| 10,000 | 0.583 | 17.14M | 0.590 | 16.94M | 1.020 | 1.75× | 1.73× |
| 100,000 | 5.871 | 17.03M | 5.948 | 16.81M | 9.639 | 1.64× | 1.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.264 | 2.39× |
| 1 | 5 | 0.220 | 1.177 | 5.35× |
| 1 | 10 | 0.425 | 1.930 | 4.54× |
| 10 | 1 | 0.045 | 0.167 | 3.73× |
| 10 | 5 | 0.210 | 0.845 | 4.03× |
| 10 | 10 | 0.428 | 2.017 | 4.72× |
| 100 | 1 | 0.052 | 0.172 | 3.29× |
| 100 | 5 | 0.221 | 0.928 | 4.21× |
| 100 | 10 | 0.463 | 2.025 | 4.38× |
| 1,000 | 1 | 0.110 | 0.289 | 2.62× |
| 1,000 | 5 | 0.315 | 1.348 | 4.28× |
| 1,000 | 10 | 0.466 | 2.763 | 5.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
