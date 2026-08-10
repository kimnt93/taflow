# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 46.98M | 0.018 | 56.93M | 0.041 | 1.95× | 2.36× |
| 10,000 | 0.188 | 53.27M | 0.161 | 61.99M | 0.173 | 0.92× | 1.07× |
| 100,000 | 1.552 | 64.42M | 1.580 | 63.28M | 1.497 | 0.96× | 0.95× |
| 1,000,000 | 15.909 | 62.86M | 17.174 | 58.23M | 14.954 | 0.94× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.142 | 0.117 | 0.82× |
| 1 | 5 | 0.313 | 0.445 | 1.42× |
| 1 | 10 | 0.535 | 0.912 | 1.71× |
| 10 | 1 | 0.063 | 0.089 | 1.41× |
| 10 | 5 | 0.245 | 0.429 | 1.75× |
| 10 | 10 | 0.549 | 0.910 | 1.66× |
| 100 | 1 | 0.066 | 0.094 | 1.41× |
| 100 | 5 | 0.259 | 0.429 | 1.65× |
| 100 | 10 | 0.536 | 0.909 | 1.70× |
| 1,000 | 1 | 0.068 | 0.105 | 1.53× |
| 1,000 | 5 | 0.265 | 0.502 | 1.90× |
| 1,000 | 10 | 0.556 | 1.063 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
