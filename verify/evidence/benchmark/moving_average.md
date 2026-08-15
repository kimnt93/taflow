# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 252.55M | 0.003 | 319.06M | 0.037 | 9.33× | 11.79× |
| 10,000 | 0.024 | 412.34M | 0.021 | 471.16M | 0.054 | 2.24× | 2.56× |
| 100,000 | 0.224 | 447.23M | 0.196 | 510.63M | 0.221 | 0.99× | 1.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.160 | 1.95× |
| 1 | 5 | 0.188 | 0.478 | 2.55× |
| 1 | 10 | 0.434 | 0.978 | 2.26× |
| 10 | 1 | 0.040 | 0.089 | 2.22× |
| 10 | 5 | 0.184 | 0.485 | 2.64× |
| 10 | 10 | 0.406 | 0.966 | 2.38× |
| 100 | 1 | 0.042 | 0.097 | 2.34× |
| 100 | 5 | 0.182 | 0.449 | 2.47× |
| 100 | 10 | 0.388 | 1.018 | 2.63× |
| 1,000 | 1 | 0.050 | 0.094 | 1.89× |
| 1,000 | 5 | 0.196 | 0.441 | 2.25× |
| 1,000 | 10 | 0.420 | 1.021 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
