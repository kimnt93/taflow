# ProjectionBands benchmark (`rolling projection mean` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.45M | 0.016 | 63.49M | 0.082 | 4.80× | 5.21× |
| 10,000 | 0.154 | 64.93M | 0.147 | 67.85M | 0.281 | 1.83× | 1.91× |
| 100,000 | 1.505 | 66.46M | 1.493 | 66.99M | 2.347 | 1.56× | 1.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.145 | 1.62× |
| 1 | 5 | 0.257 | 0.510 | 1.99× |
| 1 | 10 | 0.406 | 1.042 | 2.57× |
| 10 | 1 | 0.049 | 0.111 | 2.26× |
| 10 | 5 | 0.226 | 0.512 | 2.26× |
| 10 | 10 | 0.424 | 1.051 | 2.48× |
| 100 | 1 | 0.046 | 0.148 | 3.24× |
| 100 | 5 | 0.225 | 0.745 | 3.31× |
| 100 | 10 | 0.394 | 1.451 | 3.68× |
| 1,000 | 1 | 0.063 | 0.165 | 2.62× |
| 1,000 | 5 | 0.200 | 0.781 | 3.91× |
| 1,000 | 10 | 0.425 | 1.584 | 3.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
