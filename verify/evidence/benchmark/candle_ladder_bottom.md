# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 44.02M | 0.019 | 53.39M | 0.041 | 1.81× | 2.19× |
| 10,000 | 0.155 | 64.45M | 0.144 | 69.59M | 0.096 | 0.62× | 0.67× |
| 100,000 | 1.479 | 67.62M | 1.468 | 68.12M | 0.687 | 0.46× | 0.47× |
| 1,000,000 | 15.857 | 63.06M | 15.433 | 64.80M | 6.834 | 0.43× | 0.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.143 | 1.41× |
| 1 | 5 | 0.354 | 0.585 | 1.65× |
| 1 | 10 | 0.708 | 1.211 | 1.71× |
| 10 | 1 | 0.071 | 0.098 | 1.37× |
| 10 | 5 | 0.302 | 0.544 | 1.80× |
| 10 | 10 | 0.748 | 1.132 | 1.51× |
| 100 | 1 | 0.065 | 0.100 | 1.52× |
| 100 | 5 | 0.292 | 0.486 | 1.66× |
| 100 | 10 | 0.710 | 1.158 | 1.63× |
| 1,000 | 1 | 0.081 | 0.108 | 1.32× |
| 1,000 | 5 | 0.332 | 0.571 | 1.72× |
| 1,000 | 10 | 0.755 | 1.321 | 1.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
