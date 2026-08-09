# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.28M | 0.008 | 126.01M | 0.038 | 3.90× | 4.76× |
| 10,000 | 0.079 | 126.01M | 0.079 | 126.56M | 0.117 | 1.48× | 1.49× |
| 100,000 | 0.861 | 116.20M | 0.851 | 117.49M | 0.833 | 0.97× | 0.98× |
| 1,000,000 | 9.403 | 106.35M | 8.814 | 113.46M | 8.869 | 0.94× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.163 | 1.65× |
| 1 | 5 | 0.332 | 0.560 | 1.69× |
| 1 | 10 | 0.514 | 1.000 | 1.95× |
| 10 | 1 | 0.057 | 0.100 | 1.75× |
| 10 | 5 | 0.247 | 0.474 | 1.92× |
| 10 | 10 | 0.509 | 0.970 | 1.91× |
| 100 | 1 | 0.054 | 0.099 | 1.83× |
| 100 | 5 | 0.253 | 0.468 | 1.85× |
| 100 | 10 | 0.534 | 0.988 | 1.85× |
| 1,000 | 1 | 0.064 | 0.107 | 1.66× |
| 1,000 | 5 | 0.259 | 0.516 | 1.99× |
| 1,000 | 10 | 0.543 | 1.089 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
