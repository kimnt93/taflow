# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 211.50M | 0.004 | 268.47M | 0.031 | 6.50× | 8.25× |
| 10,000 | 0.027 | 371.52M | 0.019 | 523.16M | 0.042 | 1.54× | 2.17× |
| 100,000 | 0.181 | 551.23M | 0.159 | 628.64M | 0.121 | 0.67× | 0.76× |
| 1,000,000 | 2.112 | 473.45M | 1.693 | 590.60M | 1.061 | 0.50× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.132 | 1.18× |
| 1 | 5 | 0.284 | 0.479 | 1.69× |
| 1 | 10 | 0.455 | 0.931 | 2.05× |
| 10 | 1 | 0.047 | 0.101 | 2.16× |
| 10 | 5 | 0.273 | 0.421 | 1.54× |
| 10 | 10 | 0.575 | 1.094 | 1.90× |
| 100 | 1 | 0.049 | 0.095 | 1.94× |
| 100 | 5 | 0.222 | 0.449 | 2.02× |
| 100 | 10 | 0.492 | 0.911 | 1.85× |
| 1,000 | 1 | 0.050 | 0.090 | 1.80× |
| 1,000 | 5 | 0.217 | 0.447 | 2.07× |
| 1,000 | 10 | 0.475 | 0.943 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
