# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.55M | 0.009 | 114.34M | 0.034 | 3.78× | 3.88× |
| 10,000 | 0.070 | 142.39M | 0.064 | 155.34M | 0.090 | 1.28× | 1.40× |
| 100,000 | 0.640 | 156.30M | 0.761 | 131.33M | 0.656 | 1.02× | 0.86× |
| 1,000,000 | 6.530 | 153.14M | 6.161 | 162.30M | 6.238 | 0.96× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.109 | 1.01× |
| 1 | 5 | 0.267 | 0.441 | 1.65× |
| 1 | 10 | 0.472 | 0.867 | 1.84× |
| 10 | 1 | 0.049 | 0.088 | 1.80× |
| 10 | 5 | 0.225 | 0.399 | 1.77× |
| 10 | 10 | 0.485 | 0.884 | 1.82× |
| 100 | 1 | 0.058 | 0.088 | 1.52× |
| 100 | 5 | 0.236 | 0.427 | 1.81× |
| 100 | 10 | 0.508 | 0.921 | 1.81× |
| 1,000 | 1 | 0.053 | 0.093 | 1.75× |
| 1,000 | 5 | 0.241 | 0.465 | 1.93× |
| 1,000 | 10 | 0.531 | 1.021 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
