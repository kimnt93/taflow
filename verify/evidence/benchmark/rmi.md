# RelativeMomentumIndex benchmark (`RMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.65M | 0.009 | 107.65M | 0.186 | 19.06× | 19.99× |
| 10,000 | 0.077 | 130.38M | 0.070 | 142.88M | 0.548 | 7.14× | 7.83× |
| 100,000 | 0.695 | 143.96M | 0.672 | 148.82M | 4.324 | 6.23× | 6.43× |
| 1,000,000 | 7.114 | 140.56M | 6.634 | 150.75M | 40.132 | 5.64× | 6.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.265 | 3.10× |
| 1 | 5 | 0.334 | 1.115 | 3.34× |
| 1 | 10 | 0.510 | 2.448 | 4.80× |
| 10 | 1 | 0.051 | 0.206 | 3.99× |
| 10 | 5 | 0.213 | 1.068 | 5.02× |
| 10 | 10 | 0.461 | 2.479 | 5.37× |
| 100 | 1 | 0.054 | 0.222 | 4.08× |
| 100 | 5 | 0.231 | 1.097 | 4.76× |
| 100 | 10 | 0.481 | 2.475 | 5.15× |
| 1,000 | 1 | 0.058 | 0.259 | 4.48× |
| 1,000 | 5 | 0.228 | 1.284 | 5.62× |
| 1,000 | 10 | 0.497 | 2.902 | 5.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
