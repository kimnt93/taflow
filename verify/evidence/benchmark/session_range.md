# SessionRange benchmark (`SessionRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.56M | 0.024 | 41.21M | 0.695 | 23.31× | 28.62× |
| 10,000 | 0.187 | 53.43M | 0.163 | 61.23M | 5.082 | 27.16× | 31.12× |
| 100,000 | 1.816 | 55.07M | 1.784 | 56.06M | 58.254 | 32.08× | 32.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.277 | 2.52× |
| 1 | 5 | 0.364 | 1.255 | 3.45× |
| 1 | 10 | 0.550 | 2.496 | 4.54× |
| 10 | 1 | 0.060 | 0.244 | 4.06× |
| 10 | 5 | 0.272 | 1.385 | 5.09× |
| 10 | 10 | 0.579 | 2.573 | 4.44× |
| 100 | 1 | 0.064 | 0.296 | 4.64× |
| 100 | 5 | 0.279 | 1.664 | 5.97× |
| 100 | 10 | 0.610 | 3.124 | 5.12× |
| 1,000 | 1 | 0.089 | 1.170 | 13.15× |
| 1,000 | 5 | 0.313 | 4.340 | 13.87× |
| 1,000 | 10 | 0.642 | 8.722 | 13.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
