# CandleInNeck benchmark (`CDLINNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.68M | 0.018 | 56.92M | 0.048 | 2.29× | 2.73× |
| 10,000 | 0.159 | 62.71M | 0.148 | 67.75M | 0.124 | 0.78× | 0.84× |
| 100,000 | 1.474 | 67.82M | 1.548 | 64.61M | 1.035 | 0.70× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.124 | 1.49× |
| 1 | 5 | 0.325 | 0.465 | 1.43× |
| 1 | 10 | 0.596 | 1.007 | 1.69× |
| 10 | 1 | 0.068 | 0.088 | 1.29× |
| 10 | 5 | 0.275 | 0.465 | 1.69× |
| 10 | 10 | 0.595 | 1.087 | 1.83× |
| 100 | 1 | 0.076 | 0.102 | 1.35× |
| 100 | 5 | 0.310 | 0.515 | 1.66× |
| 100 | 10 | 0.588 | 1.023 | 1.74× |
| 1,000 | 1 | 0.083 | 0.113 | 1.36× |
| 1,000 | 5 | 0.351 | 0.541 | 1.54× |
| 1,000 | 10 | 0.603 | 1.138 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
