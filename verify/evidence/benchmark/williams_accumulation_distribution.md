# WilliamsAccumulationDistribution benchmark (`Wad` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.57M | 0.011 | 89.38M | 0.200 | 17.94× | 17.90× |
| 10,000 | 0.079 | 127.33M | 0.074 | 134.41M | 1.270 | 16.17× | 17.07× |
| 100,000 | 0.720 | 138.83M | 0.683 | 146.49M | 10.672 | 14.82× | 15.63× |
| 1,000,000 | 9.306 | 107.45M | 7.433 | 134.54M | 125.890 | 13.53× | 16.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.215 | 2.46× |
| 1 | 5 | 0.270 | 0.842 | 3.12× |
| 1 | 10 | 0.564 | 2.398 | 4.26× |
| 10 | 1 | 0.062 | 0.169 | 2.73× |
| 10 | 5 | 0.276 | 0.948 | 3.43× |
| 10 | 10 | 0.564 | 1.932 | 3.42× |
| 100 | 1 | 0.064 | 0.171 | 2.65× |
| 100 | 5 | 0.267 | 0.858 | 3.21× |
| 100 | 10 | 0.582 | 1.795 | 3.08× |
| 1,000 | 1 | 0.062 | 0.267 | 4.30× |
| 1,000 | 5 | 0.257 | 1.607 | 6.24× |
| 1,000 | 10 | 0.566 | 2.671 | 4.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
