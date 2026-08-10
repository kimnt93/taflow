# Squeeze benchmark (`squeeze` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.28M | 0.043 | 23.51M | 4.585 | 102.14× | 107.81× |
| 10,000 | 0.377 | 26.49M | 0.369 | 27.13M | 6.405 | 16.97× | 17.38× |
| 100,000 | 4.189 | 23.87M | 3.622 | 27.61M | 32.107 | 7.66× | 8.86× |
| 1,000,000 | 38.253 | 26.14M | 37.093 | 26.96M | 308.878 | 8.07× | 8.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.363 | 4.39× |
| 1 | 5 | 0.350 | 1.651 | 4.72× |
| 1 | 10 | 0.512 | 3.158 | 6.16× |
| 10 | 1 | 0.055 | 0.326 | 5.91× |
| 10 | 5 | 0.244 | 1.578 | 6.47× |
| 10 | 10 | 0.509 | 3.179 | 6.25× |
| 100 | 1 | 0.062 | 4.675 | 75.22× |
| 100 | 5 | 0.364 | 23.928 | 65.71× |
| 100 | 10 | 0.585 | 49.937 | 85.43× |
| 1,000 | 1 | 0.100 | 4.846 | 48.52× |
| 1,000 | 5 | 0.383 | 26.252 | 68.47× |
| 1,000 | 10 | 0.590 | 55.238 | 93.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
