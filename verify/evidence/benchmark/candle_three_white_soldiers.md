# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.80M | 0.019 | 52.26M | 0.044 | 1.99× | 2.27× |
| 10,000 | 0.169 | 59.00M | 0.163 | 61.16M | 0.182 | 1.07× | 1.11× |
| 100,000 | 1.719 | 58.16M | 1.704 | 58.67M | 1.625 | 0.95× | 0.95× |
| 1,000,000 | 17.571 | 56.91M | 16.713 | 59.83M | 16.086 | 0.92× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.173 | 1.93× |
| 1 | 5 | 0.360 | 0.455 | 1.26× |
| 1 | 10 | 0.536 | 0.967 | 1.80× |
| 10 | 1 | 0.061 | 0.085 | 1.40× |
| 10 | 5 | 0.244 | 0.412 | 1.69× |
| 10 | 10 | 0.517 | 0.873 | 1.69× |
| 100 | 1 | 0.059 | 0.095 | 1.61× |
| 100 | 5 | 0.300 | 0.472 | 1.57× |
| 100 | 10 | 0.612 | 1.080 | 1.77× |
| 1,000 | 1 | 0.082 | 0.107 | 1.30× |
| 1,000 | 5 | 0.279 | 0.561 | 2.01× |
| 1,000 | 10 | 0.646 | 1.150 | 1.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
