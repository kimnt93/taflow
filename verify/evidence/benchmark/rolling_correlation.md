# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.70M | 0.008 | 132.26M | 0.039 | 3.75× | 5.13× |
| 10,000 | 0.051 | 194.18M | 0.048 | 209.10M | 0.086 | 1.67× | 1.80× |
| 100,000 | 0.481 | 207.83M | 0.435 | 229.99M | 0.540 | 1.12× | 1.24× |
| 1,000,000 | 5.051 | 197.98M | 4.471 | 223.66M | 5.733 | 1.14× | 1.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.161 | 2.36× |
| 1 | 5 | 0.394 | 0.504 | 1.28× |
| 1 | 10 | 0.492 | 0.993 | 2.02× |
| 10 | 1 | 0.053 | 0.098 | 1.85× |
| 10 | 5 | 0.219 | 0.456 | 2.09× |
| 10 | 10 | 0.470 | 0.980 | 2.08× |
| 100 | 1 | 0.052 | 0.093 | 1.81× |
| 100 | 5 | 0.235 | 0.455 | 1.94× |
| 100 | 10 | 0.489 | 1.002 | 2.05× |
| 1,000 | 1 | 0.061 | 0.095 | 1.56× |
| 1,000 | 5 | 0.258 | 0.487 | 1.89× |
| 1,000 | 10 | 0.495 | 0.996 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
