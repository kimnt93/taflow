# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.34M | 0.010 | 101.28M | 0.035 | 2.65× | 3.51× |
| 10,000 | 0.075 | 132.62M | 0.071 | 140.95M | 0.107 | 1.42× | 1.51× |
| 100,000 | 0.761 | 131.40M | 0.739 | 135.38M | 0.726 | 0.95× | 0.98× |
| 1,000,000 | 7.763 | 128.82M | 7.506 | 133.22M | 7.284 | 0.94× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.103 | 1.40× |
| 1 | 5 | 0.318 | 0.461 | 1.45× |
| 1 | 10 | 0.530 | 0.900 | 1.70× |
| 10 | 1 | 0.055 | 0.089 | 1.60× |
| 10 | 5 | 0.250 | 0.427 | 1.71× |
| 10 | 10 | 0.529 | 0.907 | 1.72× |
| 100 | 1 | 0.059 | 0.090 | 1.54× |
| 100 | 5 | 0.254 | 0.427 | 1.68× |
| 100 | 10 | 0.546 | 0.898 | 1.65× |
| 1,000 | 1 | 0.060 | 0.096 | 1.59× |
| 1,000 | 5 | 0.257 | 0.476 | 1.85× |
| 1,000 | 10 | 0.580 | 0.984 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
