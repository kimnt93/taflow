# SignalDelay benchmark (`signal delay` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.04M | 0.006 | 175.78M | 0.026 | 3.86× | 4.49× |
| 10,000 | 0.039 | 256.37M | 0.035 | 283.47M | 0.030 | 0.77× | 0.86× |
| 100,000 | 0.346 | 288.92M | 0.313 | 319.59M | 0.066 | 0.19× | 0.21× |
| 1,000,000 | 3.695 | 270.60M | 3.165 | 315.93M | 0.992 | 0.27× | 0.31× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.095 | 1.46× |
| 1 | 5 | 0.267 | 0.419 | 1.57× |
| 1 | 10 | 0.465 | 0.950 | 2.04× |
| 10 | 1 | 0.049 | 0.092 | 1.86× |
| 10 | 5 | 0.213 | 0.431 | 2.03× |
| 10 | 10 | 0.482 | 0.911 | 1.89× |
| 100 | 1 | 0.045 | 0.083 | 1.86× |
| 100 | 5 | 0.212 | 0.409 | 1.93× |
| 100 | 10 | 0.465 | 0.892 | 1.92× |
| 1,000 | 1 | 0.049 | 0.087 | 1.76× |
| 1,000 | 5 | 0.222 | 0.439 | 1.97× |
| 1,000 | 10 | 0.472 | 0.909 | 1.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
