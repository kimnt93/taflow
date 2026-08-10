# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.86M | 0.010 | 96.22M | 0.042 | 2.74× | 4.00× |
| 10,000 | 0.081 | 123.84M | 0.073 | 136.49M | 0.104 | 1.29× | 1.42× |
| 100,000 | 0.789 | 126.77M | 0.730 | 137.02M | 0.759 | 0.96× | 1.04× |
| 1,000,000 | 8.519 | 117.38M | 7.391 | 135.30M | 7.324 | 0.86× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.119 | 1.43× |
| 1 | 5 | 0.282 | 0.514 | 1.82× |
| 1 | 10 | 0.614 | 0.992 | 1.62× |
| 10 | 1 | 0.054 | 0.095 | 1.77× |
| 10 | 5 | 0.237 | 0.515 | 2.17× |
| 10 | 10 | 0.604 | 1.125 | 1.86× |
| 100 | 1 | 0.060 | 0.089 | 1.47× |
| 100 | 5 | 0.271 | 0.574 | 2.12× |
| 100 | 10 | 0.576 | 1.013 | 1.76× |
| 1,000 | 1 | 0.058 | 0.096 | 1.65× |
| 1,000 | 5 | 0.239 | 0.456 | 1.91× |
| 1,000 | 10 | 0.542 | 1.124 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
