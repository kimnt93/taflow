# ArnaudLegouxMovingAverage benchmark (`ALMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.07M | 0.016 | 61.50M | 0.235 | 13.18× | 14.46× |
| 10,000 | 0.165 | 60.74M | 0.156 | 64.27M | 0.597 | 3.63× | 3.84× |
| 100,000 | 1.479 | 67.61M | 1.529 | 65.41M | 4.185 | 2.83× | 2.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.276 | 2.73× |
| 1 | 5 | 0.294 | 1.471 | 5.00× |
| 1 | 10 | 0.426 | 2.967 | 6.97× |
| 10 | 1 | 0.047 | 0.279 | 5.97× |
| 10 | 5 | 0.190 | 1.450 | 7.64× |
| 10 | 10 | 0.412 | 2.996 | 7.28× |
| 100 | 1 | 0.050 | 0.271 | 5.38× |
| 100 | 5 | 0.216 | 1.483 | 6.85× |
| 100 | 10 | 0.479 | 2.988 | 6.24× |
| 1,000 | 1 | 0.061 | 0.302 | 4.92× |
| 1,000 | 5 | 0.215 | 1.653 | 7.69× |
| 1,000 | 10 | 0.444 | 3.536 | 7.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
