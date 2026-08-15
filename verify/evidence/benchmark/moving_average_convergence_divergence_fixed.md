# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.44M | 0.005 | 206.43M | 0.051 | 7.96× | 10.57× |
| 10,000 | 0.050 | 198.23M | 0.043 | 234.70M | 0.135 | 2.67× | 3.16× |
| 100,000 | 1.476 | 67.73M | 0.390 | 256.54M | 1.628 | 1.10× | 4.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.119 | 1.99× |
| 1 | 5 | 0.297 | 0.551 | 1.85× |
| 1 | 10 | 0.447 | 1.094 | 2.45× |
| 10 | 1 | 0.041 | 0.102 | 2.47× |
| 10 | 5 | 0.178 | 0.508 | 2.85× |
| 10 | 10 | 0.430 | 1.079 | 2.51× |
| 100 | 1 | 0.051 | 0.099 | 1.94× |
| 100 | 5 | 0.209 | 0.588 | 2.81× |
| 100 | 10 | 0.497 | 1.133 | 2.28× |
| 1,000 | 1 | 0.068 | 0.146 | 2.15× |
| 1,000 | 5 | 0.275 | 0.649 | 2.36× |
| 1,000 | 10 | 0.461 | 1.246 | 2.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
