# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.92M | 0.011 | 87.30M | 0.043 | 3.36× | 3.76× |
| 10,000 | 0.076 | 131.02M | 0.075 | 132.77M | 0.094 | 1.23× | 1.25× |
| 100,000 | 0.732 | 136.66M | 0.720 | 138.98M | 0.692 | 0.95× | 0.96× |
| 1,000,000 | 8.180 | 122.24M | 7.384 | 135.42M | 7.251 | 0.89× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.123 | 1.35× |
| 1 | 5 | 0.301 | 0.521 | 1.73× |
| 1 | 10 | 0.555 | 1.035 | 1.86× |
| 10 | 1 | 0.056 | 0.099 | 1.77× |
| 10 | 5 | 0.227 | 0.486 | 2.14× |
| 10 | 10 | 0.605 | 1.087 | 1.80× |
| 100 | 1 | 0.052 | 0.092 | 1.76× |
| 100 | 5 | 0.244 | 0.452 | 1.85× |
| 100 | 10 | 0.537 | 1.088 | 2.02× |
| 1,000 | 1 | 0.062 | 0.099 | 1.61× |
| 1,000 | 5 | 0.273 | 0.519 | 1.90× |
| 1,000 | 10 | 0.497 | 1.135 | 2.28× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
