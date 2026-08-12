# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.69M | 0.013 | 77.66M | 0.041 | 2.84× | 3.16× |
| 10,000 | 0.111 | 90.04M | 0.110 | 90.52M | 0.129 | 1.16× | 1.17× |
| 100,000 | 1.104 | 90.56M | 1.167 | 85.70M | 1.080 | 0.98× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.160 | 0.137 | 0.85× |
| 1 | 5 | 0.230 | 0.479 | 2.08× |
| 1 | 10 | 0.501 | 0.950 | 1.90× |
| 10 | 1 | 0.054 | 0.097 | 1.79× |
| 10 | 5 | 0.215 | 0.422 | 1.97× |
| 10 | 10 | 0.468 | 0.948 | 2.03× |
| 100 | 1 | 0.066 | 0.103 | 1.55× |
| 100 | 5 | 0.256 | 0.432 | 1.68× |
| 100 | 10 | 0.497 | 0.952 | 1.92× |
| 1,000 | 1 | 0.061 | 0.110 | 1.81× |
| 1,000 | 5 | 0.235 | 0.529 | 2.26× |
| 1,000 | 10 | 0.508 | 0.991 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
