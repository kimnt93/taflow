# ExponentiallyWeightedCovariance benchmark (`ewm covariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.53M | 0.006 | 158.34M | 1.413 | 181.61× | 223.73× |
| 10,000 | 0.054 | 184.64M | 0.051 | 194.32M | 13.268 | 244.99× | 257.83× |
| 100,000 | 0.505 | 197.92M | 0.479 | 208.81M | 123.930 | 245.28× | 258.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.111 | 1.36× |
| 1 | 5 | 0.268 | 0.470 | 1.75× |
| 1 | 10 | 0.418 | 0.845 | 2.02× |
| 10 | 1 | 0.047 | 0.104 | 2.19× |
| 10 | 5 | 0.178 | 0.467 | 2.63× |
| 10 | 10 | 0.410 | 1.000 | 2.44× |
| 100 | 1 | 0.047 | 0.210 | 4.47× |
| 100 | 5 | 0.204 | 1.026 | 5.02× |
| 100 | 10 | 0.401 | 2.152 | 5.37× |
| 1,000 | 1 | 0.048 | 1.319 | 27.64× |
| 1,000 | 5 | 0.199 | 6.716 | 33.68× |
| 1,000 | 10 | 0.472 | 13.667 | 28.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
