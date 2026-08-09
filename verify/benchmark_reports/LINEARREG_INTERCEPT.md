# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.19M | 0.017 | 59.83M | 0.052 | 3.08× | 3.11× |
| 10,000 | 0.154 | 64.87M | 0.139 | 71.83M | 0.171 | 1.11× | 1.23× |
| 100,000 | 1.382 | 72.37M | 1.342 | 74.53M | 1.494 | 1.08× | 1.11× |
| 1,000,000 | 14.057 | 71.14M | 14.398 | 69.45M | 13.869 | 0.99× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.171 | 1.50× |
| 1 | 5 | 0.282 | 0.507 | 1.80× |
| 1 | 10 | 0.564 | 1.141 | 2.02× |
| 10 | 1 | 0.054 | 0.106 | 1.95× |
| 10 | 5 | 0.251 | 0.485 | 1.94× |
| 10 | 10 | 0.531 | 1.115 | 2.10× |
| 100 | 1 | 0.063 | 0.108 | 1.72× |
| 100 | 5 | 0.251 | 0.479 | 1.91× |
| 100 | 10 | 0.491 | 1.012 | 2.06× |
| 1,000 | 1 | 0.074 | 0.126 | 1.70× |
| 1,000 | 5 | 0.275 | 0.564 | 2.05× |
| 1,000 | 10 | 0.545 | 1.096 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
