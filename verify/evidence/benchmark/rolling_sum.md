# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.30M | 0.035 | 28.41M | 0.031 | 0.76× | 0.89× |
| 10,000 | 0.283 | 35.37M | 0.281 | 35.57M | 0.050 | 0.18× | 0.18× |
| 100,000 | 2.745 | 36.43M | 2.668 | 37.48M | 0.210 | 0.08× | 0.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.131 | 1.17× |
| 1 | 5 | 0.359 | 0.474 | 1.32× |
| 1 | 10 | 0.551 | 0.916 | 1.66× |
| 10 | 1 | 0.065 | 0.091 | 1.40× |
| 10 | 5 | 0.291 | 0.441 | 1.52× |
| 10 | 10 | 0.618 | 0.901 | 1.46× |
| 100 | 1 | 0.070 | 0.089 | 1.27× |
| 100 | 5 | 0.298 | 0.452 | 1.52× |
| 100 | 10 | 0.615 | 0.915 | 1.49× |
| 1,000 | 1 | 0.097 | 0.090 | 0.93× |
| 1,000 | 5 | 0.298 | 0.454 | 1.52× |
| 1,000 | 10 | 0.613 | 0.953 | 1.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
