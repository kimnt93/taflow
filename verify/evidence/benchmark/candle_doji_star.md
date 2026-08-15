# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.20M | 0.003 | 315.12M | 0.039 | 5.72× | 12.33× |
| 10,000 | 0.081 | 123.47M | 0.071 | 141.68M | 0.140 | 1.72× | 1.98× |
| 100,000 | 1.005 | 99.52M | 0.932 | 107.32M | 1.136 | 1.13× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.051 | 0.132 | 2.57× |
| 1 | 5 | 0.247 | 0.439 | 1.78× |
| 1 | 10 | 0.376 | 0.884 | 2.35× |
| 10 | 1 | 0.040 | 0.089 | 2.24× |
| 10 | 5 | 0.199 | 0.430 | 2.16× |
| 10 | 10 | 0.392 | 0.918 | 2.34× |
| 100 | 1 | 0.042 | 0.091 | 2.19× |
| 100 | 5 | 0.200 | 0.542 | 2.72× |
| 100 | 10 | 0.448 | 0.918 | 2.05× |
| 1,000 | 1 | 0.062 | 0.100 | 1.61× |
| 1,000 | 5 | 0.225 | 0.482 | 2.15× |
| 1,000 | 10 | 0.390 | 1.199 | 3.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
