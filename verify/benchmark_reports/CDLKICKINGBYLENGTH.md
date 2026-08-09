# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.72M | 0.010 | 103.44M | 0.039 | 3.46× | 3.99× |
| 10,000 | 0.081 | 123.75M | 0.078 | 128.03M | 0.175 | 2.17× | 2.24× |
| 100,000 | 1.047 | 95.54M | 1.052 | 95.09M | 1.488 | 1.42× | 1.41× |
| 1,000,000 | 11.243 | 88.95M | 11.079 | 90.26M | 15.033 | 1.34× | 1.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.143 | 0.99× |
| 1 | 5 | 0.281 | 0.471 | 1.68× |
| 1 | 10 | 0.528 | 0.957 | 1.81× |
| 10 | 1 | 0.052 | 0.093 | 1.80× |
| 10 | 5 | 0.246 | 0.446 | 1.81× |
| 10 | 10 | 0.507 | 0.969 | 1.91× |
| 100 | 1 | 0.053 | 0.091 | 1.70× |
| 100 | 5 | 0.254 | 0.468 | 1.84× |
| 100 | 10 | 0.523 | 0.935 | 1.79× |
| 1,000 | 1 | 0.064 | 0.108 | 1.70× |
| 1,000 | 5 | 0.246 | 0.513 | 2.08× |
| 1,000 | 10 | 0.546 | 1.139 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
