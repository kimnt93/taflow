# CandleMorningDojiStar benchmark (`CDLMORNINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.36M | 0.003 | 291.40M | 0.038 | 5.68× | 10.93× |
| 10,000 | 0.055 | 180.41M | 0.051 | 194.47M | 0.122 | 2.19× | 2.36× |
| 100,000 | 0.831 | 120.30M | 0.829 | 120.62M | 0.799 | 0.96× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.145 | 1.60× |
| 1 | 5 | 0.200 | 0.472 | 2.37× |
| 1 | 10 | 0.391 | 0.916 | 2.34× |
| 10 | 1 | 0.047 | 0.094 | 1.98× |
| 10 | 5 | 0.179 | 0.514 | 2.87× |
| 10 | 10 | 0.408 | 0.918 | 2.25× |
| 100 | 1 | 0.042 | 0.088 | 2.08× |
| 100 | 5 | 0.172 | 0.428 | 2.48× |
| 100 | 10 | 0.408 | 0.945 | 2.31× |
| 1,000 | 1 | 0.049 | 0.098 | 1.99× |
| 1,000 | 5 | 0.185 | 0.471 | 2.54× |
| 1,000 | 10 | 0.415 | 1.020 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
