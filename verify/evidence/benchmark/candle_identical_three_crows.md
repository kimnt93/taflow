# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.23M | 0.026 | 39.08M | 0.042 | 1.40× | 1.65× |
| 10,000 | 0.185 | 53.96M | 0.176 | 56.89M | 0.138 | 0.74× | 0.78× |
| 100,000 | 1.913 | 52.27M | 1.687 | 59.28M | 1.127 | 0.59× | 0.67× |
| 1,000,000 | 19.116 | 52.31M | 16.139 | 61.96M | 10.948 | 0.57× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.136 | 1.46× |
| 1 | 5 | 0.392 | 0.588 | 1.50× |
| 1 | 10 | 0.647 | 1.163 | 1.80× |
| 10 | 1 | 0.080 | 0.108 | 1.34× |
| 10 | 5 | 0.372 | 0.558 | 1.50× |
| 10 | 10 | 0.642 | 1.109 | 1.73× |
| 100 | 1 | 0.066 | 0.094 | 1.41× |
| 100 | 5 | 0.373 | 0.584 | 1.56× |
| 100 | 10 | 0.683 | 1.080 | 1.58× |
| 1,000 | 1 | 0.084 | 0.110 | 1.31× |
| 1,000 | 5 | 0.395 | 0.778 | 1.97× |
| 1,000 | 10 | 0.691 | 1.259 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
