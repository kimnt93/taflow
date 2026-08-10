# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.33M | 0.015 | 68.59M | 0.039 | 2.21× | 2.70× |
| 10,000 | 0.148 | 67.61M | 0.149 | 67.32M | 0.164 | 1.11× | 1.10× |
| 100,000 | 1.523 | 65.66M | 1.523 | 65.66M | 1.396 | 0.92× | 0.92× |
| 1,000,000 | 15.864 | 63.04M | 15.192 | 65.82M | 13.906 | 0.88× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.132 | 1.56× |
| 1 | 5 | 0.339 | 0.547 | 1.61× |
| 1 | 10 | 0.540 | 0.963 | 1.78× |
| 10 | 1 | 0.055 | 0.092 | 1.68× |
| 10 | 5 | 0.253 | 0.433 | 1.71× |
| 10 | 10 | 0.572 | 0.897 | 1.57× |
| 100 | 1 | 0.056 | 0.092 | 1.66× |
| 100 | 5 | 0.265 | 0.438 | 1.65× |
| 100 | 10 | 0.552 | 0.904 | 1.64× |
| 1,000 | 1 | 0.071 | 0.102 | 1.44× |
| 1,000 | 5 | 0.263 | 0.499 | 1.90× |
| 1,000 | 10 | 0.588 | 1.039 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
