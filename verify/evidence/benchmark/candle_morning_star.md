# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 64.90M | 0.013 | 78.46M | 0.039 | 2.52× | 3.05× |
| 10,000 | 0.141 | 71.00M | 0.141 | 71.09M | 0.116 | 0.83× | 0.83× |
| 100,000 | 1.364 | 73.31M | 1.339 | 74.66M | 0.858 | 0.63× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.131 | 1.93× |
| 1 | 5 | 0.268 | 0.482 | 1.80× |
| 1 | 10 | 0.385 | 1.034 | 2.69× |
| 10 | 1 | 0.045 | 0.101 | 2.25× |
| 10 | 5 | 0.182 | 0.489 | 2.69× |
| 10 | 10 | 0.399 | 0.962 | 2.41× |
| 100 | 1 | 0.043 | 0.094 | 2.20× |
| 100 | 5 | 0.211 | 0.509 | 2.41× |
| 100 | 10 | 0.405 | 0.990 | 2.45× |
| 1,000 | 1 | 0.057 | 0.109 | 1.89× |
| 1,000 | 5 | 0.213 | 0.515 | 2.41× |
| 1,000 | 10 | 0.467 | 1.070 | 2.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
