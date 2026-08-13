# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.100 | 10.04M | 0.091 | 10.98M | 0.039 | 0.39× | 0.43× |
| 10,000 | 0.848 | 11.79M | 0.796 | 12.56M | 0.122 | 0.14× | 0.15× |
| 100,000 | 7.900 | 12.66M | 8.315 | 12.03M | 1.009 | 0.13× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.169 | 0.167 | 0.99× |
| 1 | 5 | 0.500 | 0.458 | 0.92× |
| 1 | 10 | 0.635 | 0.912 | 1.44× |
| 10 | 1 | 0.069 | 0.083 | 1.21× |
| 10 | 5 | 0.309 | 0.415 | 1.34× |
| 10 | 10 | 0.623 | 0.904 | 1.45× |
| 100 | 1 | 0.075 | 0.099 | 1.32× |
| 100 | 5 | 0.312 | 0.420 | 1.34× |
| 100 | 10 | 0.653 | 0.873 | 1.34× |
| 1,000 | 1 | 0.149 | 0.096 | 0.64× |
| 1,000 | 5 | 0.345 | 0.471 | 1.37× |
| 1,000 | 10 | 0.700 | 1.013 | 1.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
