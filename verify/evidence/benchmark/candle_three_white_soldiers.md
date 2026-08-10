# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 40.94M | 0.022 | 44.99M | 0.069 | 2.83× | 3.11× |
| 10,000 | 0.184 | 54.22M | 0.177 | 56.51M | 0.196 | 1.06× | 1.11× |
| 100,000 | 1.759 | 56.87M | 1.768 | 56.55M | 2.103 | 1.20× | 1.19× |
| 1,000,000 | 18.611 | 53.73M | 17.699 | 56.50M | 16.665 | 0.90× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.123 | 1.24× |
| 1 | 5 | 0.271 | 0.541 | 2.00× |
| 1 | 10 | 0.651 | 0.948 | 1.46× |
| 10 | 1 | 0.056 | 0.089 | 1.57× |
| 10 | 5 | 0.258 | 0.425 | 1.65× |
| 10 | 10 | 0.626 | 1.037 | 1.66× |
| 100 | 1 | 0.056 | 0.090 | 1.61× |
| 100 | 5 | 0.281 | 0.458 | 1.63× |
| 100 | 10 | 0.559 | 1.097 | 1.96× |
| 1,000 | 1 | 0.101 | 0.121 | 1.20× |
| 1,000 | 5 | 0.305 | 0.560 | 1.83× |
| 1,000 | 10 | 0.613 | 1.177 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
