# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.95M | 0.004 | 226.83M | 0.046 | 7.32× | 10.38× |
| 10,000 | 0.033 | 298.65M | 0.026 | 381.07M | 0.129 | 3.86× | 4.92× |
| 100,000 | 0.287 | 348.13M | 0.223 | 447.65M | 1.040 | 3.62× | 4.66× |
| 1,000,000 | 12.444 | 80.36M | 2.376 | 420.86M | 10.782 | 0.87× | 4.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.137 | 1.31× |
| 1 | 5 | 0.308 | 0.523 | 1.70× |
| 1 | 10 | 0.492 | 1.013 | 2.06× |
| 10 | 1 | 0.048 | 0.097 | 2.04× |
| 10 | 5 | 0.216 | 0.506 | 2.35× |
| 10 | 10 | 0.489 | 1.067 | 2.18× |
| 100 | 1 | 0.059 | 0.115 | 1.94× |
| 100 | 5 | 0.224 | 0.494 | 2.21× |
| 100 | 10 | 0.472 | 1.021 | 2.16× |
| 1,000 | 1 | 0.063 | 0.109 | 1.73× |
| 1,000 | 5 | 0.229 | 0.540 | 2.36× |
| 1,000 | 10 | 0.493 | 1.106 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
