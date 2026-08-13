# RollingLinearRegressionSlope benchmark (`LINEARREG_SLOPE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.185 | 5.41M | 0.199 | 5.02M | 0.039 | 0.21× | 0.19× |
| 10,000 | 1.668 | 5.99M | 1.673 | 5.98M | 0.132 | 0.08× | 0.08× |
| 100,000 | 16.574 | 6.03M | 16.577 | 6.03M | 1.024 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.128 | 0.133 | 1.04× |
| 1 | 5 | 0.362 | 0.472 | 1.30× |
| 1 | 10 | 0.637 | 0.994 | 1.56× |
| 10 | 1 | 0.068 | 0.092 | 1.36× |
| 10 | 5 | 0.289 | 0.422 | 1.46× |
| 10 | 10 | 0.608 | 0.960 | 1.58× |
| 100 | 1 | 0.088 | 0.090 | 1.03× |
| 100 | 5 | 0.314 | 0.441 | 1.41× |
| 100 | 10 | 0.643 | 0.931 | 1.45× |
| 1,000 | 1 | 0.240 | 0.103 | 0.43× |
| 1,000 | 5 | 0.390 | 0.474 | 1.22× |
| 1,000 | 10 | 0.729 | 1.039 | 1.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
