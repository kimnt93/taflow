# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.63M | 0.003 | 307.73M | 0.038 | 5.78× | 11.73× |
| 10,000 | 0.085 | 117.25M | 0.078 | 128.23M | 0.137 | 1.61× | 1.76× |
| 100,000 | 0.971 | 102.99M | 0.939 | 106.53M | 1.076 | 1.11× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.106 | 1.56× |
| 1 | 5 | 0.247 | 0.482 | 1.95× |
| 1 | 10 | 0.393 | 0.910 | 2.31× |
| 10 | 1 | 0.045 | 0.101 | 2.24× |
| 10 | 5 | 0.190 | 0.422 | 2.22× |
| 10 | 10 | 0.387 | 0.903 | 2.33× |
| 100 | 1 | 0.043 | 0.093 | 2.19× |
| 100 | 5 | 0.194 | 0.446 | 2.30× |
| 100 | 10 | 0.431 | 0.924 | 2.15× |
| 1,000 | 1 | 0.054 | 0.101 | 1.86× |
| 1,000 | 5 | 0.191 | 0.472 | 2.46× |
| 1,000 | 10 | 0.443 | 1.081 | 2.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
