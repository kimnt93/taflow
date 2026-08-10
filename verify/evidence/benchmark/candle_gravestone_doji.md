# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.15M | 0.011 | 87.87M | 0.044 | 2.79× | 3.88× |
| 10,000 | 0.088 | 113.12M | 0.089 | 112.27M | 0.113 | 1.28× | 1.27× |
| 100,000 | 0.911 | 109.79M | 0.938 | 106.61M | 0.901 | 0.99× | 0.96× |
| 1,000,000 | 8.981 | 111.35M | 9.003 | 111.07M | 8.884 | 0.99× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.150 | 1.23× |
| 1 | 5 | 0.315 | 0.509 | 1.61× |
| 1 | 10 | 0.561 | 1.086 | 1.94× |
| 10 | 1 | 0.059 | 0.088 | 1.48× |
| 10 | 5 | 0.301 | 0.520 | 1.72× |
| 10 | 10 | 0.601 | 1.055 | 1.76× |
| 100 | 1 | 0.070 | 0.106 | 1.51× |
| 100 | 5 | 0.317 | 0.559 | 1.76× |
| 100 | 10 | 0.696 | 1.147 | 1.65× |
| 1,000 | 1 | 0.089 | 0.114 | 1.28× |
| 1,000 | 5 | 0.327 | 0.532 | 1.63× |
| 1,000 | 10 | 0.769 | 1.129 | 1.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
