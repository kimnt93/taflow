# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 160.53M | 0.005 | 183.26M | 0.032 | 5.18× | 5.91× |
| 10,000 | 0.037 | 268.88M | 0.035 | 289.04M | 0.052 | 1.39× | 1.49× |
| 100,000 | 0.342 | 292.64M | 0.319 | 313.03M | 0.228 | 0.67× | 0.71× |
| 1,000,000 | 4.074 | 245.47M | 3.471 | 288.06M | 2.182 | 0.54× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.128 | 1.17× |
| 1 | 5 | 0.234 | 0.455 | 1.94× |
| 1 | 10 | 0.492 | 0.990 | 2.01× |
| 10 | 1 | 0.051 | 0.092 | 1.79× |
| 10 | 5 | 0.222 | 0.423 | 1.91× |
| 10 | 10 | 0.461 | 0.951 | 2.06× |
| 100 | 1 | 0.055 | 0.102 | 1.86× |
| 100 | 5 | 0.248 | 0.444 | 1.79× |
| 100 | 10 | 0.480 | 0.939 | 1.96× |
| 1,000 | 1 | 0.053 | 0.103 | 1.93× |
| 1,000 | 5 | 0.218 | 0.514 | 2.36× |
| 1,000 | 10 | 0.507 | 0.962 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
