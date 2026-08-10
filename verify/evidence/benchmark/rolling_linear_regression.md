# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 40.83M | 0.019 | 51.65M | 0.053 | 2.18× | 2.76× |
| 10,000 | 0.215 | 46.58M | 0.147 | 67.83M | 0.174 | 0.81× | 1.18× |
| 100,000 | 1.577 | 63.43M | 1.417 | 70.56M | 1.426 | 0.90× | 1.01× |
| 1,000,000 | 15.149 | 66.01M | 14.890 | 67.16M | 13.799 | 0.91× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.148 | 1.40× |
| 1 | 5 | 0.275 | 0.513 | 1.86× |
| 1 | 10 | 0.549 | 1.065 | 1.94× |
| 10 | 1 | 0.051 | 0.100 | 1.95× |
| 10 | 5 | 0.232 | 0.453 | 1.96× |
| 10 | 10 | 0.503 | 1.016 | 2.02× |
| 100 | 1 | 0.051 | 0.098 | 1.91× |
| 100 | 5 | 0.252 | 0.450 | 1.79× |
| 100 | 10 | 0.528 | 1.014 | 1.92× |
| 1,000 | 1 | 0.085 | 0.108 | 1.28× |
| 1,000 | 5 | 0.251 | 0.545 | 2.17× |
| 1,000 | 10 | 0.499 | 1.123 | 2.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
