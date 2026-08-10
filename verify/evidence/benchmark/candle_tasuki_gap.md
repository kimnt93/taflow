# CandleTasukiGap benchmark (`CDLTASUKIGAP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.07M | 0.017 | 57.84M | 0.046 | 2.15× | 2.64× |
| 10,000 | 0.177 | 56.64M | 0.159 | 62.96M | 0.185 | 1.05× | 1.17× |
| 100,000 | 1.744 | 57.33M | 1.772 | 56.44M | 1.602 | 0.92× | 0.90× |
| 1,000,000 | 17.918 | 55.81M | 17.209 | 58.11M | 16.139 | 0.90× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.150 | 1.75× |
| 1 | 5 | 0.294 | 0.448 | 1.53× |
| 1 | 10 | 0.556 | 1.052 | 1.89× |
| 10 | 1 | 0.071 | 0.094 | 1.33× |
| 10 | 5 | 0.265 | 0.470 | 1.78× |
| 10 | 10 | 0.524 | 0.976 | 1.86× |
| 100 | 1 | 0.060 | 0.114 | 1.90× |
| 100 | 5 | 0.329 | 0.534 | 1.62× |
| 100 | 10 | 0.635 | 0.931 | 1.47× |
| 1,000 | 1 | 0.068 | 0.107 | 1.57× |
| 1,000 | 5 | 0.313 | 0.619 | 1.98× |
| 1,000 | 10 | 0.582 | 1.094 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
